use crossbeam::channel::{Receiver, Sender};
use crossbeam::thread;
use lazy_static::lazy_static;
use reqwest::Url;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

#[cfg(not(test))] //For the "mock" at the end of file
use super::downloader;

use super::args;
use super::disk;
use super::dom;

static DEFAULT_CAPACITY: usize = 128;

lazy_static! {
    static ref VISITED_URLS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
pub struct Scraper {
    args: args::Args,
    transmitter: Sender<Url>,
    receiver: Receiver<Url>,
    downloader: downloader::Downloader,
    depth_level: usize,
}

impl Scraper {
    /// Create a new scraper with command line options
    pub fn new(args: args::Args) -> Scraper {
        let (tx, rx) = crossbeam::channel::unbounded();

        let mut scraper = Scraper {
            downloader: downloader::Downloader::new(args.tries),
            args: args,
            transmitter: tx,
            receiver: rx,
            depth_level: 0,
        };

        scraper
    }

    /// Push a new URL into the channel
    fn push(transmitter: &Sender<Url>, url: Url) { // FIXME: Send String + Path instead of URL
        let mut visited_urls = VISITED_URLS.lock().unwrap();

        match visited_urls.contains_key(url.as_str()) {
            false => {
                visited_urls.insert(url.to_string(), disk::url_to_path(&url));
                match transmitter.send(url) {
                    Ok(_) => {}
                    Err(e) => panic!("{}", e),
                };
            }
            true => (),
        }
    }

    /// Process a single URL
    fn handle_url(
        scraper: &Scraper,
        transmitter: &Sender<Url>,
        url: Url,
    ) {
        let page = scraper.downloader.get(&url).unwrap();
        let dom = dom::Dom::new(&page);

        let new_urls = dom.find_urls_as_strings();
        let new_urls = new_urls
            .into_iter()
            .filter(|candidate| Scraper::should_visit(candidate, &url));

        for new_url_string in new_urls {
            let new_full_url = url.join(&new_url_string).unwrap();
            Scraper::push(transmitter, new_full_url.clone());

            let visited_urls = VISITED_URLS.lock().unwrap();

            new_url_string.clear();
            new_url_string
                .push_str(visited_urls.get(new_full_url.as_str()).unwrap());
        }

        let visited_urls = VISITED_URLS.lock().unwrap();

        disk::save_file(
            visited_urls.get(url.as_str()).unwrap(),
            &dom.serialize(),
            &scraper.args.output,
        );

        println!("{} has been downloaded", url);
    }

    /// Run through the channel and complete it
    pub fn run(&mut self) {
        /* Push the origin URL through the channel */
        match self.transmitter.send(self.args.origin.clone()) {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        };

        thread::scope(|thread_scope| {
            for i in 0..self.args.jobs {
                let o0 = self.args.output.clone(); // FIXME:
                let tx_clone = self.transmitter.clone();
                let rx_clone = self.receiver.clone();
                let self_clone = &self;

                thread_scope.spawn(move |_| {
                    loop {
                        match rx_clone.recv() {
                            Err(_) => continue, // FIXME: Sleep
                            Ok(url) => {
                                Scraper::handle_url(&self_clone, &tx_clone, url);
                            }
                        };
                    }
                });
            }
        })
        .unwrap();
    }

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

        assert_eq!(s.queue.len(), 2); //Base url + depth delimiter
        assert_eq!(
            s.queue.pop_front().unwrap().unwrap().to_string(),
            "https://example.com/"
        );
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

        assert!(!s.visited_urls.contains_key("https://example.net"));
        assert!(!s.visited_urls.contains_key("https://no-no-no.com"));
        assert!(s.visited_urls.contains_key("https://fake_start.net/a_file"));
        assert!(s
            .visited_urls
            .contains_key("https://fake_start.net/dir/nested/file"));
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

        assert!(!s.visited_urls.contains_key("https://example.net"));
        assert!(!s.visited_urls.contains_key("https://no-no-no.com"));
        assert!(!s.visited_urls.contains_key("https://fake_start.net/a_file"));
        assert!(!s
            .visited_urls
            .contains_key("https://fake_start.net/dir/nested/file"));
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

        assert!(s.visited_urls.contains_key("https://fake_start.net/a_file"));
        assert!(s
            .visited_urls
            .contains_key("https://fake_start.net/an_answer_file"));
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
