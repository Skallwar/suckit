mod downloader;
mod parser;

use reqwest::Url;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "url", required = true, parse(try_from_str))]
    origin: Url,
}

fn main() {
    let opt = Opt::from_args();
    let page = downloader::download_url(opt.origin).unwrap();
    let urls = parser::find_urls(page);

    println!("{:?}", urls);
}
