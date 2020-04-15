use std::path::PathBuf;

use reqwest::Url;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(name = "url", parse(try_from_str))]
    pub origin: Url,

    #[structopt(short, long, parse(from_os_str))]
    pub output: Option<PathBuf>,

    #[structopt(short, long, default_value = "1")]
    pub jobs: usize,
}

impl Args {
    pub fn collect() -> Args {
        Args::from_args()
    }
}
