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

    info!("Hey there, suckit speaking");
    warn!("Start sucking");

    scraper.run();

    error!("sucking complete !");
}
