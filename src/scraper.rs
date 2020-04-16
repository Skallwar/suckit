use crossbeam::channel::{Receiver, Sender, TryRecvError};
use crossbeam::thread;
use lazy_static::lazy_static;
use reqwest::Url;

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;
use std::time;

#[cfg(not(test))] //For the "mock" at the end of file
use super::downloader;

use super::args;
use super::disk;
use super::dom;

/// Maximum number of empty recv() from the channel
static MAX_EMPTY_RECEIVES: usize = 10;

/// Sleep duration on empty recv()
static SLEEP_MILLIS: u64 = 100;
static SLEEP_DURATION: time::Duration = time::Duration::from_millis(SLEEP_MILLIS);

lazy_static! {
    /// HashSet containing visited URLs
    static ref VISITED_URLS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

lazy_static! {
    /// HashMap containing URLs and their path on the disk
    static ref PATH_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
pub struct Scraper {
    args: args::Args,
    transmitter: Sender<(Url, usize)>,
    receiver: Receiver<(Url, usize)>,
    downloader: downloader::Downloader,
}

impl Scraper {
    /// Create a new scraper with command line options
    pub fn new(args: args::Args) -> Scraper {
        let (tx, rx) = crossbeam::channel::unbounded();

        Scraper {
            downloader: downloader::Downloader::new(args.tries),
            args: args,
            transmitter: tx,
            receiver: rx,
        }
    }

    /// Add an URL to the path_map HashMap
    fn map_url(url: &Url, path: String) -> bool {
        let mut path_map = PATH_MAP.lock().unwrap();

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
    fn fix_domtree(old_url_str: &mut String, new_url: &Url) {
        let path_map = PATH_MAP.lock().unwrap();

        old_url_str.clear();
        old_url_str.push_str(path_map.get(new_url.as_str()).unwrap());
    }

    /// Process a single URL
    fn handle_url(scraper: &Scraper, transmitter: &Sender<(Url, usize)>, url: Url, depth: usize) {
        let page = scraper.downloader.get(&url).unwrap();
        let dom = dom::Dom::new(&page);

        dom.find_urls_as_strings()
            .into_iter()
            .filter(|candidate| Scraper::should_visit(candidate, &url))
            .for_each(|next_url| {
                let next_full_url = url.join(&next_url).unwrap();
                match Scraper::map_url(&next_full_url, disk::url_to_path(&next_full_url)) {
                    true => {
                        if depth < scraper.args.depth {
                            Scraper::push(transmitter, next_full_url.clone(), depth + 1);
                        }
                    },
                    false => (),
                };

                Scraper::fix_domtree(next_url, &next_full_url);
            });

        let path_map = PATH_MAP.lock().unwrap();

        disk::save_file(
            path_map.get(url.as_str()).unwrap(),
            &dom.serialize(),
            &scraper.args.output,
        );

        VISITED_URLS.lock().unwrap().insert(url.to_string());

        println!("{} has been downloaded", url);
    }

    /// Run through the channel and complete it
    pub fn run(&mut self) {
        /* Push the origin URL and depth (0) through the channel */
        Scraper::map_url(&self.args.origin, disk::url_to_path(&self.args.origin));
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
        };
        let mut s = Scraper::new(args);
    }

    #[test]
    fn run() {
        let args = args::Args {
            origin: Url::parse("https://fake_start.net/").unwrap(),
            output: Some(PathBuf::from("/tmp")),
            jobs: 1,
            tries: 1,
            depth: 5,
        };
        let mut s = Scraper::new(args);

        s.run();

        let visited_urls = VISITED_URLS.lock().unwrap();

        assert!(!visited_urls.contains("https://example.net"));
        assert!(!visited_urls.contains("https://no-no-no.com"));
        assert!(visited_urls.contains("https://fake_start.net/a_file"));
        assert!(visited_urls
            .contains("https://fake_start.net/dir/nested/file"));
    }

    #[test]
    fn depth() {
        let args = args::Args {
            origin: Url::parse("https://fake_start.net/").unwrap(),
            output: Some(PathBuf::from("/tmp")),
            jobs: 1,
            tries: 1,
            depth: 0,
        };
        let mut s = Scraper::new(args);

        s.run();

        let visited_urls = VISITED_URLS.lock().unwrap();

        assert!(!visited_urls.contains("https://example.net"));
        assert!(!visited_urls.contains("https://no-no-no.com"));
        assert!(!visited_urls.contains("https://fake_start.net/an_answer_file"));
        assert!(!visited_urls
            .contains("https://fake_start.net/dir/nested/file"));
    }

    #[test]
    fn run_recursive() {
        let args = args::Args {
            origin: Url::parse("https://fake_start.net/").unwrap(),
            output: Some(PathBuf::from("/tmp")),
            jobs: 1,
            tries: 1,
            depth: 5,
        };
        let mut s = Scraper::new(args);

        s.run();

        let visited_urls = VISITED_URLS.lock().unwrap();

        assert!(visited_urls.contains("https://fake_start.net/a_file"));
        assert!(visited_urls
            .contains("https://fake_start.net/an_answer_file"));
    }
}

#[cfg(test)]
mod downloader {
    static TEST_BEG: &str = "<!DOCTYPE html>
<html>
    <body>
        <p>Absolute<a href=\"https://no-no-no.com\"></a></p>
        <p>Relative<a href=\"a_file\"></a></p>
        <p>Relative nested<a href=\"dir/nested/file\"></a></p>
    </body>
</html>
";

    static TEST_ANS: &str = "<!DOCTYPE html>
<html>
    <body>
        <p>Relative<a href=\"an_answer_file\"></a></p>
    </body>
</html>
";

    pub struct Downloader {}

    impl Downloader {
        pub fn new(_tries: usize) -> Downloader {
            Downloader {}
        }

        pub fn get(&self, url: &reqwest::Url) -> Result<String, reqwest::Error> {
            let mut res = String::from("");

            match url.as_str() == "https://fake_start.net/" {
                true => res = String::from(TEST_BEG),
                false => {}
            }

            match url.as_str() == "https://fake_start.net/a_file" {
                true => res = String::from(TEST_ANS),
                false => {}
            }

            Ok(res)
        }
    }
}
