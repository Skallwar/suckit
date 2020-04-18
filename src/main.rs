mod args;
mod disk;
mod dom;
mod downloader;
mod scraper;
mod logger;

use scraper::Scraper;

fn main() {
    let args = args::Args::collect();

    let mut scraper = Scraper::new(args);

    scraper.run();

}
