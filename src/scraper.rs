use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Mutex;
use std::time;

use crossbeam::channel::{Receiver, Sender, TryRecvError};
use crossbeam::thread;
use encoding_rs::Encoding;
use lazy_static::lazy_static;
use pathdiff;
use rand::Rng;
use regex::Regex;
use url::Url;

use crate::{error, info, warn};

use super::args;
use super::disk;
use super::dom;
use super::downloader;
use super::response;
use super::url_helper;

/// Maximum number of empty recv() from the channel
static MAX_EMPTY_RECEIVES: usize = 10;

/// If args.depth is this, it will download everything
static INFINITE_DEPTH: i32 = -1;

/// Sleep duration on empty recv()
static SLEEP_MILLIS: u64 = 100;
static SLEEP_DURATION: time::Duration = time::Duration::from_millis(SLEEP_MILLIS);

/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
pub struct Scraper {
    args: args::Args,
    transmitter: Sender<(Url, i32, i32)>,
    receiver: Receiver<(Url, i32, i32)>,
    downloader: downloader::Downloader,
    visited_urls: Mutex<HashSet<String>>,
    path_map: Mutex<HashMap<String, String>>,
}

impl Scraper {
    /// Create a new scraper with command line options
    pub fn new(args: args::Args) -> Scraper {
        let (tx, rx) = crossbeam::channel::unbounded();

        Scraper {
            downloader: downloader::Downloader::new(
                args.tries,
                &args.user_agent,
                &args.auth,
                &args.origin,
            ),
            args,
            transmitter: tx,
            receiver: rx,
            visited_urls: Mutex::new(HashSet::new()),
            path_map: Mutex::new(HashMap::new()),
        }
    }

    /// Add an URL to the path_map HashMap and return if it was inserted or not
    fn map_url_path(&self, url: &Url, path: String) -> bool {
        let mut path_map = self.path_map.lock().unwrap();

        if !path_map.contains_key(url.as_str()) {
            path_map.insert(url.to_string(), path);
            true
        } else {
            false
        }
    }

    /// Push a new URL into the channel
    fn push(transmitter: &Sender<(Url, i32, i32)>, url: Url, depth: i32, ext_depth: i32) {
        if let Err(e) = transmitter.send((url, depth, ext_depth)) {
            error!("Couldn't push to channel ! {}", e);
        }
    }

    /// Fix the URLs contained in the DOM-tree so they point to each other relatively
    fn fix_domtree(&self, dom_url: &mut String, source_path: &str, dest_path: &str) {
        let source_path_parent = Path::new(source_path).parent().unwrap().to_str().unwrap(); //Unwrap should be safe, there will alway be at least .../index.html
        let diff_path = pathdiff::diff_paths(dest_path, source_path_parent).unwrap();
        let relative_path = diff_path.as_path().to_str().unwrap();

        dom_url.clear();
        dom_url.push_str(&relative_path);
    }

    /// Find the charset of the webpage. ``data`` is not a String as this might not be utf8.
    /// Returned String is lower cased
    /// This is a hack and should be check in case of a bug
    fn find_charset(data: &[u8], http_charset: Option<String>) -> Option<String> {
        lazy_static! {
            static ref CHARSET_REGEX: Regex =
                Regex::new("<meta.*charset\\s*=\\s*\"?([^\"\\s;>]+).*>").unwrap();
        }

        // We don't know the real charset yet. We hope that the charset is ASCII
        // compatible, because Rust String are in UTF-8 (also ASCII compatible).
        let data_utf8 = unsafe { String::from_utf8_unchecked(Vec::from(data)) };
        let captures = CHARSET_REGEX.captures_iter(&data_utf8).next();

        // We use the first one, hopping we are in the <head> of the page... or if nothing is found
        // we used the http charset (if any).
        captures
            .map(|first| String::from(first.get(1).unwrap().as_str().to_lowercase()))
            .or(http_charset)
    }

    /// Proceed to convert the data in utf8.
    fn charset_convert(
        data: &[u8],
        charset_source: &'static Encoding,
        charset_dest: &'static Encoding,
    ) -> Vec<u8> {
        let decode_result = charset_source.decode(data);
        let decode_bytes = decode_result.0.borrow();

        let encode_result = charset_dest.encode(decode_bytes);
        let encode_bytes = encode_result.0.into_owned();

        encode_bytes
    }

    /// Check if the charset require conversion
    fn needs_charset_conversion(charset: &str) -> bool {
        match charset {
            "utf-8" => false,
            _ => true,
        }
    }

    /// Proces an html file: add new url to the chanel and prepare for offline navigation
    fn handle_html(
        scraper: &Scraper,
        transmitter: &Sender<(Url, i32, i32)>,
        url: &Url,
        depth: i32,
        ext_depth: i32,
        data: &[u8],
        http_charset: Option<String>,
    ) -> Vec<u8> {
        let charset_source_str = match Self::find_charset(data, http_charset) {
            Some(s) => s,
            None => {
                warn!("Charset not found for {}, defaulting to UTF-8", url);
                String::from("utf-8")
            }
        };

        let need_charset_conversion = Self::needs_charset_conversion(&charset_source_str);

        let charset_source = match encoding_rs::Encoding::for_label(&charset_source_str.as_bytes())
        {
            Some(encoder) => encoder,
            None => {
                warn!(
                    "Charset {} not supported for {}, defaulting to UTF-8",
                    charset_source_str, url
                );
                encoding_rs::UTF_8
            }
        };
        let charset_utf8 = encoding_rs::UTF_8;
        let utf8_data = if need_charset_conversion {
            Self::charset_convert(data, charset_source, charset_utf8)
        } else {
            Vec::from(data)
        };

        let dom = dom::Dom::new(&String::from_utf8_lossy(&utf8_data).into_owned());
        let source_path = match scraper.path_map.lock().unwrap().get(url.as_str()) {
            Some(path) => path.clone(),
            None => error!("Url {} was not found in the path map", url.as_str()),
        };

        dom.find_urls_as_strings()
            .into_iter()
            .filter(|candidate| Scraper::should_visit(candidate))
            .for_each(|next_url| {
                let url_to_parse = Scraper::normalize_url(next_url.clone());

                let mut next_full_url = match url.join(url_to_parse.as_str()) {
                    Ok(url) => url,
                    Err(e) => panic!("Failed to parse url: {} | Error: {}", next_url, e),
                };

                next_full_url.set_fragment(None);
                let path = url_helper::to_path(&next_full_url);

                if scraper.map_url_path(&next_full_url, path.clone()) {
                    if !Scraper::is_on_another_domain(&next_url, &url) {
                        // If we are determining for a local domain
                        if scraper.args.depth == INFINITE_DEPTH || depth < scraper.args.depth {
                            Scraper::push(transmitter, next_full_url, depth + 1, ext_depth);
                        }
                    } else {
                        // If we are determining for an external domain
                        if scraper.args.ext_depth == INFINITE_DEPTH
                            || ext_depth < scraper.args.ext_depth
                        {
                            Scraper::push(transmitter, next_full_url, depth, ext_depth + 1);
                        }
                    }
                }

                scraper.fix_domtree(next_url, &source_path, &path);
            });

        let utf8_data = dom.serialize().into_bytes();

        if need_charset_conversion {
            Self::charset_convert(&utf8_data, charset_utf8, charset_source)
        } else {
            utf8_data
        }
    }

    /// Process a single URL
    fn handle_url(
        scraper: &Scraper,
        transmitter: &Sender<(Url, i32, i32)>,
        url: Url,
        depth: i32,
        ext_depth: i32,
    ) {
        match scraper.downloader.get(&url) {
            Ok(response) => {
                let data = match response.data {
                    response::ResponseData::Html(data) => Scraper::handle_html(
                        scraper,
                        transmitter,
                        &url,
                        depth,
                        ext_depth,
                        &data,
                        response.charset,
                    ),
                    response::ResponseData::Other(data) => data,
                };

                // Create a scope to unlock path_map automagicly
                {
                    let path_map = scraper.path_map.lock().unwrap();
                    let path = path_map.get(url.as_str()).unwrap();

                    if !scraper.args.dry_run
                        && !scraper.args.exclude.is_match(url.as_str())
                        && scraper.args.include.is_match(url.as_str())
                    {
                        match response.filename {
                            Some(filename) => {
                                disk::save_file(&filename, &data, &scraper.args.output);
                                disk::symlink(path, &filename, &scraper.args.output);
                            }
                            None => {
                                disk::save_file(path, &data, &scraper.args.output);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                if !scraper.args.continue_on_error {
                    error!("Couldn't download a page, {:?}", e);
                } else {
                    warn!("Couldn't download a page, {:?}", e);
                }
            }
        }

        scraper.visited_urls.lock().unwrap().insert(url.to_string());

        if scraper.args.verbose {
            info!("Visited: {}", url);
        }
    }

    /// Run through the channel and complete it
    pub fn run(&mut self) {
        /* Push the origin URL and depth (0) through the channel */
        self.map_url_path(&self.args.origin, url_helper::to_path(&self.args.origin));
        Scraper::push(&self.transmitter, self.args.origin.clone(), 0, 0);

        thread::scope(|thread_scope| {
            for _ in 0..self.args.jobs {
                let tx = self.transmitter.clone();
                let rx = self.receiver.clone();
                let self_clone = &self;

                thread_scope.spawn(move |_| {
                    let mut counter = 0;
                    // For a random delay
                    let mut rng = rand::thread_rng();

                    while counter < MAX_EMPTY_RECEIVES {
                        match rx.try_recv() {
                            Err(e) => match e {
                                TryRecvError::Empty => {
                                    counter += 1;
                                    std::thread::sleep(SLEEP_DURATION);
                                }
                                TryRecvError::Disconnected => panic!("{}", e),
                            },
                            Ok((url, depth, ext_depth)) => {
                                counter = 0;
                                Scraper::handle_url(&self_clone, &tx, url, depth, ext_depth);
                                self_clone.sleep(&mut rng);
                            }
                        }
                    }
                });
            }
        })
        .unwrap();
    }

    /// Sleep the thread for a variable amount of seconds to avoid getting banned
    fn sleep(&self, rng: &mut rand::rngs::ThreadRng) {
        let base_delay = self.args.delay;
        let random_range = self.args.random_range;

        if base_delay == 0 && random_range == 0 {
            return;
        }

        // delay_range+1 because gen_range is exclusive on the upper limit
        let rand_delay_secs = rng.gen_range(0..random_range + 1);
        let delay_duration = time::Duration::from_secs(base_delay + rand_delay_secs);
        std::thread::sleep(delay_duration);
    }

    /// If a URL should be visited (ignores `mail:`, `javascript:` and other pseudo-links)
    fn should_visit(url: &str) -> bool {
        match Url::parse(url) {
            /* The given candidate is a valid URL, and not a relative path to
             * the next one. Therefore, we have to check if this URL is valid.
             * If it is, we should visit it.
             */
            Ok(not_ok) => not_ok.has_host() && !not_ok.cannot_be_a_base(),

            /* Since we couldn't parse this "URL", then it must be a relative
             * path or a malformed URL. If the URL is malformed, then it will
             * be handled during the join() call in run() */
            Err(_) => true,
        }
    }

    /// Replaces `///` with `//`
    /// And `//` with `https://`
    /// Without this function, if url is `///<domain>.<extension>/`, the app crashes.
    fn normalize_url(url: String) -> String {
        if url.starts_with("///") {
            return url.replacen("///", "https://", 1);
        } else if url.starts_with("//") {
            return url.replacen("//", "https://", 1);
        }
        url
    }

    /// If the URL leads to another domain
    fn is_on_another_domain(url: &str, base: &Url) -> bool {
        let real_url = Scraper::normalize_url(String::from(url));

        match Url::parse(real_url.as_str()) {
            /* The given candidate is a valid URL, and not a relative path to
             * the next one. Therefore, we have to check if this URL belongs
             * to the same domain as our current URL. If the candidate has the
             * same domain as our base, and the depth condition is satisfied,
             * then we should visit it,  */
            Ok(not_ok) => not_ok.domain() != base.domain(),

            /* Since we couldn't parse this "URL", then it must be a relative
             * path or a malformed URL. If the URL is malformed, then it will
             * be handled during the join() call in run() */
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use regex::Regex;

    use super::*;

    #[test]
    fn test_zero_delay_range() {
        let args = args::Args {
            origin: Url::parse("https://example.com/").unwrap(),
            output: Some(PathBuf::from("/tmp")),
            jobs: 1,
            tries: 1,
            depth: 5,
            ext_depth: 0,
            delay: 0,
            user_agent: "suckit".to_string(),
            random_range: 0,
            verbose: true,
            include: Regex::new("jpg").unwrap(),
            exclude: Regex::new("png").unwrap(),
            auth: Vec::new(),
            continue_on_error: true,
            dry_run: false,
        };

        let _ = Scraper::new(args);
    }

    #[test]
    fn test_non_zero_delay_range() {
        let args = args::Args {
            origin: Url::parse("https://example.com/").unwrap(),
            output: Some(PathBuf::from("/tmp")),
            jobs: 1,
            tries: 1,
            depth: 5,
            ext_depth: 0,
            delay: 2,
            user_agent: "suckit".to_string(),
            random_range: 5,
            verbose: true,
            include: Regex::new("jpg").unwrap(),
            exclude: Regex::new("png").unwrap(),
            auth: Vec::new(),
            continue_on_error: true,
            dry_run: false,
        };

        let _ = Scraper::new(args);
    }
}
