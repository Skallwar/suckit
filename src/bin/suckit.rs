use suckit::args::Args;
use suckit::scraper::Scraper;

fn main() {
    let args = Args::collect();

    let mut scraper = Scraper::new(args);

    scraper.run();
}
