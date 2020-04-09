use reqwest::Url;

use std::collections::VecDeque;

use super::downloader;
use super::parser;

static DEFAULT_CAPACITY: usize = 128;

/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
pub struct Scraper {
    #[cfg(not(test))]
    queue: VecDeque<Url>,
    #[cfg(test)]
    pub queue: VecDeque<Url>,
}

impl Scraper {
    /// Creates a new scraper with no tasks
    pub fn new() -> Scraper {
        let new_scraper = Scraper {
            queue: VecDeque::with_capacity(DEFAULT_CAPACITY),
        };

        new_scraper
    }

    /// Add an element for the scraper to handle
    pub fn push(&mut self, url: Url) {
        self.queue.push_back(url);
    }

    /// Remove an element from the scraper's queue
    pub fn pop(&mut self) -> Option<Url> {
        self.queue.pop_front()
    }

    /// Run through the queue and complete it
    pub fn run(&mut self) {
        // TODO: Add multithreading handling
        while !self.queue.is_empty() {
            match self.pop() {
                None => panic!("unhandled data race, entered the loop with emtpy queue"),
                Some(url) => {
                    let page = downloader::download_url(url.clone()).unwrap();
                    let new_urls = parser::find_urls(page);

                    // FIXME: Add proper error handling
                    new_urls.into_iter().for_each(|x| self.push(url.join(&x).unwrap()));
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_one() {
        let mut s = Scraper::new();

        s.push(Url::parse("https://example.com/").unwrap());

        assert_eq!(s.queue.len(), 1);
    }

    #[test]
    fn pop_one() {
        let mut s = Scraper::new();

        s.push(Url::parse("https://example.com/").unwrap());

        match s.pop() {
            None => assert!(false),
            Some(url) => assert_eq!(url.to_string(), "https://example.com/"),
        };
    }

    #[test]
    fn pop_empty() {
        let mut s = Scraper::new();

        match s.pop() {
            None => assert!(true),
            Some(invalid) => assert!(false),
        };
    }

    #[test]
    fn order() {
        let mut s = Scraper::new();

        s.push(Url::parse("http://0.com/").unwrap());
        s.push(Url::parse("http://1.com/").unwrap());
        s.push(Url::parse("http://2.com/").unwrap());

        match s.pop() {
            None => assert!(false),
            Some(url) => assert_eq!(url.to_string(), "http://0.com/"),
        }

        match s.pop() {
            None => assert!(false),
            Some(url) => assert_eq!(url.to_string(), "http://1.com/"),
        }

        match s.pop() {
            None => assert!(false),
            Some(url) => assert_eq!(url.to_string(), "http://2.com/"),
        }
    }
}
