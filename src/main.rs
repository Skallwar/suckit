extern crate reqwest;

mod downloader;

use structopt::StructOpt;
use reqwest::Url;

#[derive(Debug, StructOpt)]
struct Opt
{
    #[structopt(name = "url", required = true, parse(try_from_str))]
    origin: Url,
}

fn main() {
    let opt = Opt::from_args();
    println!("{}", downloader::download_url(opt.origin).unwrap());
}

