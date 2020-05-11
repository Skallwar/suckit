use std::path::PathBuf;

use regex::Regex;
use structopt::StructOpt;
use url::Url;

///CLI arguments
#[derive(Debug, StructOpt)]
pub struct Args {
    ///Entry point of scraping
    #[structopt(
        name = "url",
        parse(try_from_str),
        help = "Entry point of the scraping"
    )]
    pub origin: Url,

    ///Output directory
    #[structopt(short, long, parse(from_os_str), help = "Output directory")]
    pub output: Option<PathBuf>,

    ///Number of threads/workers
    #[structopt(
        short,
        long,
        default_value = "1",
        help = "Maximum number of threads to use concurrently"
    )]
    pub jobs: usize,

    ///Max depth of scraping recursion
    #[structopt(
        short,
        long,
        default_value = "-1",
        help = "Maximum recursion depth to reach when visiting. -1 is the default and will go as far as it can"
    )]
    pub depth: i32,

    ///Number of retries when downloading a page fails
    #[structopt(
        short,
        long,
        default_value = "20",
        help = "Maximum amount of retries on download failure"
    )]
    pub tries: usize,

    ///Show all logs
    #[structopt(
        short,
        long,
        help = "Enable more information regarding the scraping process"
    )]
    pub verbose: bool,

    /// The least seconds of delay between downloads
    #[structopt(
        long,
        default_value = "0",
        help = "Add a delay in seconds between downloads to reduce the likelihood of getting banned"
    )]
    pub delay: u64,

    /// The max seconds of delay between downloads
    #[structopt(
        long,
        default_value = "0",
        help = "Generate an extra random delay between downloads, from 0 to this number. This is added to the base delay seconds"
    )]
    pub random_range: u64,

    /// User agent to be used to send requests
    #[structopt(
        short,
        long,
        default_value = "suckit",
        help = "User agent to be used for sending requests"
    )]
    pub user_agent: String,

    /// Regex filter to limit saving pages to only matched ones
    #[structopt(
        short,
        long,
        default_value = ".*",
        parse(try_from_str = parse_regex),
        help = "Regex filter to limit to only saving pages that match this expression"
    )]
    pub include: Regex,

    /// Regex filter to limit saving pages to only matched ones
    #[structopt(
        short,
        long,
        default_value = "$^",
        parse(try_from_str = parse_regex),
        help = "Regex filter to exclude saving pages that match this expression"
    )]
    pub exclude: Regex,
}

impl Args {
    ///Collect args
    pub fn collect() -> Args {
        Args::from_args()
    }
}

fn parse_regex(src: &str) -> Result<Regex, regex::Error> {
    Regex::new(src)
}
