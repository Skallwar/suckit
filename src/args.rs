use std::path::PathBuf;

use structopt::StructOpt;
use url::Url;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(name = "url", parse(try_from_str), help = "URL to start scraping from")]
    pub origin: Url,

    #[structopt(short, long, parse(from_os_str), help = "Output directory")]
    pub output: Option<PathBuf>,

    #[structopt(short, long, default_value = "1", help = "Maximum number of threads to use concurrently")]
    pub jobs: usize,

    #[structopt(short, long, default_value = "5", help = "Maximum recursion depth to reach when visiting")]
    pub depth: usize,

    #[structopt(short, long, default_value = "20", help = "Maximum amount of retries on download failure")]
    pub tries: usize,

    #[structopt(short, long, help = "Enable more information regarding the download")]
    pub verbose: bool,
}

impl Args {
    pub fn collect() -> Args {
        Args::from_args()
    }
}
