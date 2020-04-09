mod downloader;
mod parser;
mod scraper;

use scraper::Scraper;

use reqwest::Url;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "url", required = true, parse(try_from_str))]
    origin: Url,
}

fn main() {
    let opt = Opt::from_args();

    let mut scraper = Scraper::new(opt.origin);

    scraper.run();
}
