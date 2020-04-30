use std::path::PathBuf;

use structopt::StructOpt;
use url::Url;

use super::url_helper;

///CLI arguments
#[derive(Debug, StructOpt)]
pub struct Args {
    ///Entry point of scraping
    #[structopt(name = "url", parse(try_from_str = url_helper::str_to_url))]
    pub origin: Url,

    ///Output directory
    #[structopt(short, long, parse(from_os_str))]
    pub output: Option<PathBuf>,

    ///Number of threads/workers
    #[structopt(short, long, default_value = "1")]
    pub jobs: usize,

    ///Max depth of scraping recursion
    #[structopt(short, long, default_value = "5")]
    pub depth: usize,

    ///Number of retries when download of a page fails
    #[structopt(short, long, default_value = "20")]
    pub tries: usize,

    ///Show logs
    #[structopt(short, long)]
    pub verbose: bool,
}

impl Args {
    ///Collect args
    pub fn collect() -> Args {
        Args::from_args()
    }
}
