use reqwest::Url;

use std::collections::HashMap;
use std::collections::VecDeque;

#[cfg(not(test))] //For the "mock" at the end of file
use super::downloader;

use super::args;
use super::disk;
use super::dom;

static DEFAULT_CAPACITY: usize = 128;

/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
pub struct Scraper {
    args: args::Args,
    queue: VecDeque<Option<Url>>,
    visited_urls: HashMap<String, String>,
    downloader: downloader::Downloader,
    depth_level: usize,
}

impl Scraper {
    /// Create a new scraper with command line options
    pub fn new(args: args::Args) -> Scraper {
        let mut scraper = Scraper {
            queue: VecDeque::with_capacity(DEFAULT_CAPACITY),
            visited_urls: HashMap::new(),
            downloader: downloader::Downloader::new(args.tries),
            args: args,
            depth_level: 0,
        };

        scraper.queue_init(scraper.args.origin.clone());

        scraper
    }

    /* Use wrappers functions for consistency */

    fn push_depth_delimiter(&mut self) {
        self.queue.push_back(None);
    }

    fn queue_init(&mut self, url: Url) {
        //Entry point + depth delimiter
        self.push(url);
        self.push_depth_delimiter();
    }

    fn push(&mut self, url: Url) {
        match self.visited_urls.contains_key(url.as_str()) {
            false => {
                self.visited_urls
                    .insert(url.to_string(), disk::url_to_path(&url));
                self.queue.push_back(Some(url));
            }
            true => (),
        }
    }

    fn pop(&mut self) -> Option<Url> {
        //Only a depth delimiter remaining
        if self.queue.len() == 1 {
            return None;
        }

        match self.queue.pop_front() {
            Some(url) => match url {
                Some(url) => Some(url),
                None => {
                    self.depth_level += 1;
                    self.push_depth_delimiter();
                    self.pop()
                }
            },
            None => None,
        }
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

    /// Run through a small queue for which multithreading isn't as beneficial
    fn run_small_queue(&mut self) {
            match self.pop() {
                None => return,
                Some(url) => {
                    let page = self.downloader.get(&url).unwrap();
                    let dom = dom::Dom::new(&page);

                    if self.depth_level < self.args.depth {
                        let new_urls = dom.find_urls_as_strings();
                        let new_urls = new_urls
                            .into_iter()
                            .filter(|candidate| Scraper::should_visit(candidate, &url));

                        for new_url_string in new_urls {
                            let new_full_url = url.join(&new_url_string).unwrap();

                            self.push(new_full_url.clone());
                            new_url_string.clear();
                            new_url_string
                                .push_str(self.visited_urls.get(new_full_url.as_str()).unwrap());
                        }
                    }

                    disk::save_file(
                        self.visited_urls.get(url.as_str()).unwrap(),
                        &dom.serialize(),
                        &self.args.output,
                    );

                    println!("{} has been downloaded", url);
                }
            };

        self.run();
    }

    /// Run through the queue and complete it
    pub fn run(&mut self) {
        if self.queue.len() < self.args.jobs {
            self.run_small_queue();
        } else {
            self.run_small_queue();
            /*
            for i in 0..self.args.threads {
                let url = self.pop().unwrap(); //FIXME: Needs an Rc
                thread::spawn(move || self.handle_url(url));
            }
            */
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
