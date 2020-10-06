use crossbeam::channel::{Receiver, Sender, TryRecvError};
use crossbeam::thread;
use url::Url;

use std::collections::HashMap;
use std::collections::HashSet;
use std::process;
use std::sync::Mutex;
use std::time;

use rand::Rng;

use super::downloader;

use super::args;
use super::disk;
use super::dom;
use super::response;
use super::url_helper;

use crate::{error, info};

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
    transmitter: Sender<(Url, i32)>,
    receiver: Receiver<(Url, i32)>,
    downloader: downloader::Downloader,
    visited_urls: Mutex<HashSet<String>>,
    path_map: Mutex<HashMap<String, String>>,
}

impl Scraper {
    /// Create a new scraper with command line options
    pub fn new(args: args::Args) -> Scraper {
        let (tx, rx) = crossbeam::channel::unbounded();

        Scraper {
            downloader: downloader::Downloader::new(args.tries, &args.user_agent, &args.auth),
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
    fn push(transmitter: &Sender<(Url, i32)>, url: Url, depth: i32) {
        if let Err(e) = transmitter.send((url, depth)) {
            error!("Couldn't push to channel ! {}", e);
        }
    }

    /// Fix the URLs contained in the DOM-tree so they point to each other
    fn fix_domtree(&self, old_url_str: &mut String, new_url: &Url) {
        let path_map = self.path_map.lock().unwrap();
        let path = path_map.get(new_url.as_str()).unwrap();

        let new_url_str = url_helper::encode(path);

        old_url_str.clear();
        old_url_str.push_str(&new_url_str);
    }

    ///Proces an html file: add new url to the chanel and prepare for offline navigation
    fn handle_html(
        scraper: &Scraper,
        transmitter: &Sender<(Url, i32)>,
        url: &Url,
        depth: i32,
        data: &str,
    ) -> Vec<u8> {
        let dom = dom::Dom::new(data);

        dom.find_urls_as_strings()
            .into_iter()
            .filter(|candidate| Scraper::should_visit(candidate, &url))
            .for_each(|next_url| {
                let next_full_url = url.join(&next_url).unwrap();
                let path = url_helper::to_path(&next_full_url);

                if scraper.map_url_path(&next_full_url, path)
                    && (scraper.args.depth == INFINITE_DEPTH || depth < scraper.args.depth)
                {
                    Scraper::push(transmitter, next_full_url.clone(), depth + 1);
                }

                scraper.fix_domtree(next_url, &next_full_url);
            });

        dom.serialize().into_bytes()
    }

    /// Process a single URL
    fn handle_url(scraper: &Scraper, transmitter: &Sender<(Url, i32)>, url: Url, depth: i32) {
        match scraper.downloader.get(&url) {
            Ok(response) => {
                let data = match response.data {
                    response::ResponseData::Html(data) => {
                        Scraper::handle_html(scraper, transmitter, &url, depth, &data)
                    }
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
                println!("Couldn't download a page, {:?}", e);
                if !scraper.args.continue_on_error {
                    process::exit(1);
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
        Scraper::push(&self.transmitter, self.args.origin.clone(), 0);

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
                            Ok((url, depth)) => {
                                counter = 0;
                                Scraper::handle_url(&self_clone, &tx, url, depth);
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
        let rand_delay_secs = rng.gen_range(0, random_range + 1);
        let delay_duration = time::Duration::from_secs(base_delay + rand_delay_secs);
        std::thread::sleep(delay_duration);
    }

    /// If a URL should be visited, or does it belong to another domain
    fn should_visit(url: &str, base: &Url) -> bool {
        match Url::parse(url) {
            /* The given candidate is a valid URL, and not a relative path to
             * the next one. Therefore, we have to check if this URL belongs
             * to the same domain as our current URL. If the candidate has the
             * same domain as our base, then we should visit it */
            Ok(not_ok) => not_ok.domain() == base.domain(),

            /* Since we couldn't parse this "URL", then it must be a relative
             * path or a malformed URL. If the URL is malformed, then it will
             * be handled during the join() call in run() */
            Err(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::path::PathBuf;

    #[test]
    fn test_zero_delay_range() {
        let args = args::Args {
            origin: Url::parse("https://example.com/").unwrap(),
            output: Some(PathBuf::from("/tmp")),
            jobs: 1,
            tries: 1,
            depth: 5,
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
