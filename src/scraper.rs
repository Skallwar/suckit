use reqwest::Url;

use std::collections::VecDeque;

use super::downloader;
use super::parser;

static DEFAULT_CAPACITY: usize = 128;

/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
pub struct Scraper {
    queue: VecDeque<Url>,
}

impl Scraper {
    /// Creates a new scraper with no tasks
    pub fn new(url: Url) -> Scraper {
        let mut new_scraper = Scraper {
            queue: VecDeque::with_capacity(DEFAULT_CAPACITY),
        };

        new_scraper.queue.push_front(url);

        new_scraper
    }

    fn should_visit(url: &str, base: &Url) -> bool {
        match Url::parse(url) {
            /* The given candidate is a valid URL, and not a relative path to
             * the next one. Therefore, we have to check if this URL belongs
             * to the same domain as our current URL */
            Ok(not_ok) => not_ok.domain() == base.domain(),

            /* Since we couldn't parse this "URL", then it must be a relative
             * path or a malformed URL. If the URL is malformed, then it will
             * be handled during the join() call in run() */
            Err(_) => true
        }
    }

    /// Run through the queue and complete it
    pub fn run(&mut self) {
        // TODO: Add multithreading handling
        while !self.queue.is_empty() {
            match self.queue.pop_front() {
                None => panic!("unhandled data race, entered the loop with emtpy queue"),
                Some(url) => {
                    dbg!(url.as_str());
                    let page = downloader::download_url(url.clone()).unwrap();
                    let new_urls = parser::find_urls(page);

                    new_urls
                        .into_iter()
                        .filter(|candidate| Scraper::should_visit(candidate, &url))
                        .for_each(|x| self.queue.push_back(url.join(&x).unwrap()));
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut s = Scraper::new(Url::parse("https://example.com/").unwrap());

        assert_eq!(s.queue.len(), 1);
        assert_eq!(
            s.queue.pop_front().unwrap().to_string(),
            "https://example.com/"
        );
    }
}
