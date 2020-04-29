use crossbeam::channel::{Receiver, Sender, TryRecvError};
use crossbeam::thread;
use url::Url;

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;
use std::time;

use super::downloader;

use super::args;
use super::disk;
use super::dom;
use super::url_helper;
use super::response;

use crate::info;

/// Maximum number of empty recv() from the channel
static MAX_EMPTY_RECEIVES: usize = 10;

/// Sleep duration on empty recv()
static SLEEP_MILLIS: u64 = 100;
static SLEEP_DURATION: time::Duration = time::Duration::from_millis(SLEEP_MILLIS);

/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
pub struct Scraper {
    args: args::Args,
    transmitter: Sender<(Url, usize)>,
    receiver: Receiver<(Url, usize)>,
    downloader: downloader::Downloader,
    visited_urls: Mutex<HashSet<String>>,
    path_map: Mutex<HashMap<String, String>>,
}

impl Scraper {
    /// Create a new scraper with command line options
    pub fn new(args: args::Args) -> Scraper {
        let (tx, rx) = crossbeam::channel::unbounded();

        Scraper {
            downloader: downloader::Downloader::new(args.tries),
            args,
            transmitter: tx,
            receiver: rx,
            visited_urls: Mutex::new(HashSet::new()),
            path_map: Mutex::new(HashMap::new()),
        }
    }

    /// Add an URL to the path_map HashMap
    fn map_url(&self, url: &Url, path: String) -> bool {
        let mut path_map = self.path_map.lock().unwrap();

        match path_map.contains_key(url.as_str()) {
            false => {
                path_map.insert(url.to_string(), path);
                true
            }
            true => false,
        }
    }

    /// Push a new URL into the channel
    fn push(transmitter: &Sender<(Url, usize)>, url: Url, depth: usize) {
        match transmitter.send((url, depth)) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
    }

    /// Fix the URLs contained in the DOM-tree so they point to each other
    fn fix_domtree(&self, old_url_str: &mut String, new_url: &Url) {
        let path_map = self.path_map.lock().unwrap();
        let new_url_str = url_helper::str_percent_encode(path_map.get(new_url.as_str()).unwrap());

        old_url_str.clear();
        old_url_str.push_str(&new_url_str);
    }

    fn handle_html(
        scraper: &Scraper,
        transmitter: &Sender<(Url, usize)>,
        url: &Url,
        depth: usize,
        data: &str,
    ) -> Vec<u8> {
        let dom = dom::Dom::new(data);

        dom.find_urls_as_strings()
            .into_iter()
            .filter(|candidate| Scraper::should_visit(candidate, &url))
            .for_each(|next_url| {
                let next_full_url = url.join(&next_url).unwrap();
                match scraper.map_url(&next_full_url, url_helper::url_to_path(&next_full_url)) {
                    true => {
                        if depth < scraper.args.depth {
                            Scraper::push(transmitter, next_full_url.clone(), depth + 1);
                        }
                    }
                    false => (),
                };

                scraper.fix_domtree(next_url, &next_full_url);
            });

        dom.serialize().into_bytes()
    }

    /// Process a single URL
    fn handle_url(scraper: &Scraper, transmitter: &Sender<(Url, usize)>, url: Url, depth: usize) {
        let response = scraper.downloader.get(&url).unwrap();

        let data = match response.get_data() {
            response::ResponseData::Html(data) => {
                Scraper::handle_html(scraper, transmitter, &url, depth, data)
            }
            response::ResponseData::Other(data) => data.to_vec(),
        };

        match response.get_filename() {
            Some(filename) => {
                disk::save_file(filename, &data, &scraper.args.output);

                let path_map = scraper.path_map.lock().unwrap();
                disk::symlink(
                    filename,
                    path_map.get(url.as_str()).unwrap(),
                    &scraper.args.output,
                );
            }
            None => {
                let path_map = scraper.path_map.lock().unwrap();
                disk::save_file(
                    path_map.get(url.as_str()).unwrap(),
                    &data,
                    &scraper.args.output,
                );
            }
        }

        scraper.visited_urls.lock().unwrap().insert(url.to_string());

        if scraper.args.verbose {
            info!("Downloaded {}", url);
        }
    }

    /// Run through the channel and complete it
    pub fn run(&mut self) {
        /* Push the origin URL and depth (0) through the channel */
        self.map_url(
            &self.args.origin,
            url_helper::url_to_path(&self.args.origin),
        );
        Scraper::push(&self.transmitter, self.args.origin.clone(), 0);

        thread::scope(|thread_scope| {
            for _ in 0..self.args.jobs {
                let tx = self.transmitter.clone();
                let rx = self.receiver.clone();
                let self_clone = &self;

                thread_scope.spawn(move |_| {
                    let mut counter = 0;

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
                            }
                        }
                    }
                });
            }
        })
        .unwrap();
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
    use std::path::PathBuf;

    #[test]
    fn new() {
        let args = args::Args {
            origin: Url::parse("https://example.com/").unwrap(),
            output: Some(PathBuf::from("/tmp")),
            jobs: 1,
            tries: 1,
            depth: 5,
            verbose: true,
        };

        let _ = Scraper::new(args);
    }
}
