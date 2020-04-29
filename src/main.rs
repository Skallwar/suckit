mod args;
mod disk;
mod dom;
mod downloader;
mod logger;
mod scraper;

use scraper::Scraper;

fn main() {
    let args = args::Args::collect();

    let mut scraper = Scraper::new(args);

    scraper.run();
}
