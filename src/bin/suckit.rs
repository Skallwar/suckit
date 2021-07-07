mod args;
mod disk;
mod dom;
mod downloader;
mod logger;
mod response;
mod scraper;
mod url_helper;

use scraper::Scraper;

fn main() {
    let args = args::Args::collect();

    let mut scraper = Scraper::new(args);

    scraper.run();
}
