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
        help = "Maximum recursion depth to reach when visiting. Default is -1 (infinity)"
    )]
    pub depth: i32,

    #[structopt(
        long,
        default_value = "0",
        help = "Maximum recursion depth to reach when visiting external domains. Default is 0. -1 means infinity"
    )]
    pub ext_depth: i32,

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

    /// Cookie header
    #[structopt(
        long,
        default_value = "",
        help = "Cookie to send with each request, format: key1=value1;key2=value2"
    )]
    pub cookie: String,

    /// Regex filter to limit visiting pages to only matched ones
    #[structopt(
    long,
    default_value = ".*",
    parse(try_from_str = parse_regex),
    help = "Regex filter to limit to only visiting pages that match this expression"
    )]
    pub include_visit: Regex,

    /// Regex filter to limit visiting pages to only matched ones
    #[structopt(
    long,
    default_value = "$^",
    parse(try_from_str = parse_regex),
    help = "Regex filter to exclude visiting pages that match this expression"
    )]
    pub exclude_visit: Regex,

    /// Regex filter to limit saving pages to only matched ones
    #[structopt(
    short,
    long,
    default_value = ".*",
    parse(try_from_str = parse_regex),
    help = "Regex filter to limit to only saving pages that match this expression"
    )]
    pub include_download: Regex,

    /// Regex filter to limit saving pages to only matched ones
    #[structopt(
    short,
    long,
    default_value = "$^",
    parse(try_from_str = parse_regex),
    help = "Regex filter to exclude saving pages that match this expression"
    )]
    pub exclude_download: Regex,

    /// If set, set the visit filter to the values of the download filter
    #[structopt(
        long,
        help = "Use the dowload filter in/exclude regexes for visiting as well"
    )]
    pub visit_filter_is_download_filter: bool,

    /// HTTP basic authentication credentials
    #[structopt(
        short,
        long,
        use_delimiter = true,
        value_delimiter = " ",
        help = "HTTP basic authentication credentials space-separated as \"username password host\". Can be repeated for multiple credentials as \"u1 p1 h1 u2 p2 h2\""
    )]
    pub auth: Vec<String>,

    /// Decides if we should bail out on download error (like, too many redirects)
    #[structopt(short, long, help = "Flag to enable or disable exit on error")]
    pub continue_on_error: bool,

    /// If set, run without saving anything to the disk
    #[structopt(long, help = "Do everything without saving the files to the disk")]
    pub dry_run: bool,

    #[structopt(long, help = "Dissable SSL certificates verification")]
    pub disable_certs_checks: bool,
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
