use reqwest::Url;

use std::collections::VecDeque;

use super::disk;
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

    /// Run through the queue and complete it
    pub fn run(&mut self) {
        // TODO: Add multithreading handling
        while !self.queue.is_empty() {
            match self.queue.pop_front() {
                None => panic!("unhandled data race, entered the loop with emtpy queue"),
                Some(url) => {
                    let page = downloader::download_url(url.clone()).unwrap();
                    let new_urls = parser::find_urls(&page);

                    // FIXME: Add proper error handling ? suckit should probably
                    // stop and display something meaningful if the new base
                    // cannot be appended to the old one
                    new_urls
                        .into_iter()
                        .for_each(|x| self.queue.push_back(url.join(&x).unwrap()));

                    disk::save_to_disk(&url, &page);

                    println!("{} has been downloaded", url);
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
