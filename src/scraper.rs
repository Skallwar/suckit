use std::collections::VecDeque;
use reqwest::Url;

static DEFAULT_CAPACITY: usize = 128;

/// Producer and Consumer data structure. Handles the incoming requests and
/// adds more as new URLs are found
struct Scraper {
    queue: VecDeque<Url>,
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
    pub fn pop(&mut self) -> Url {
        let res = self.queue.pop_front().unwrap();

        res
    }

    /// Run through the queue and complete it
    pub fn run() {}
}
