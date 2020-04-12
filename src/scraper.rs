use reqwest::Url;

use std::collections::HashSet;
use std::collections::VecDeque;

#[cfg(not(test))] //For the "mock" at the end of file
use super::downloader::Downloader;

use super::args;
use super::disk;
use super::parser;

static DEFAULT_CAPACITY: usize = 128;

/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
pub struct Scraper {
    args: args::Args,
    queue: VecDeque<Url>,
    visited_urls: HashSet<String>,
    downloader: Downloader,
}

impl Scraper {
    /// Create a new scraper with command line options
    pub fn new(args: args::Args) -> Scraper {
        let mut scraper = Scraper {
            args: args,
            queue: VecDeque::with_capacity(DEFAULT_CAPACITY),
            visited_urls: HashSet::new(),
            downloader: Downloader::new(),
        };

        scraper.push(scraper.args.origin.clone());

        scraper
    }

    /* Use wrappers functions for consistency */

    fn push(&mut self, url: Url) {
        match self.visited_urls.contains(url.as_str()) {
            false => {
                self.visited_urls.insert(url.to_string());
                self.queue.push_back(url);
            }
            true => {}
        }
    }

    fn pop(&mut self) -> Option<Url> {
        self.queue.pop_front()
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

    /// Run through the queue and complete it
    pub fn run(&mut self) {
        // TODO: Add multithreading handling
        while !self.queue.is_empty() {
            match self.pop() {
                None => panic!("unhandled data race, entered the loop with empty queue"),
                Some(url) => {
                    let page = self.downloader.get(url.clone()).unwrap();
                    let new_urls = parser::find_urls(&page);

                    new_urls
                        .into_iter()
                        .filter(|candidate| Scraper::should_visit(candidate, &url))
                        .for_each(|x| self.push(url.join(&x).unwrap()));

                    disk::save_file(&url, &page, &self.args.output);

                    println!("{} has been downloaded", url);
                }
            };
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
        };
        let mut s = Scraper::new(args);

        assert_eq!(s.queue.len(), 1);
        assert_eq!(
            s.queue.pop_front().unwrap().to_string(),
            "https://example.com/"
        );
    }

    #[test]
    fn run() {
        let args = args::Args {
            origin: Url::parse("https://fake_start.net/").unwrap(),
            output: Some(PathBuf::from("/tmp")),
        };
        let mut s = Scraper::new(args);

        s.run();

        assert!(!s.visited_urls.contains("https://example.net"));
        assert!(!s.visited_urls.contains("https://no-no-no.com"));
        assert!(s.visited_urls.contains("https://fake_start.net/a_file"));
        assert!(s
            .visited_urls
            .contains("https://fake_start.net/dir/nested/file"));
    }
}

#[cfg(test)]
mod downloader {
    static SIMPLE_BODY: &str = "<!DOCTYPE html>
<html>
    <body>
        <p>Absolute <a href=\"https://no-no-no.com\"></a></p>
        <p>Relative <a href=\"a_file\"></a></p>
        <p>Relative nested <a href=\"dir/nested/file\"></a></p>
    </body>
</html>
";

    pub struct Downloader {}

    impl Downloader {
        pub fn new() -> Downloader {
            Downloader {}
        }

        pub fn get(&self, url: reqwest::Url) -> Result<String, reqwest::Error> {
            match url.as_str() == "https://fake_start.net/" {
                true => Ok(String::from(SIMPLE_BODY)),
                false => Ok(String::from("")),
            }
        }
    }
}
