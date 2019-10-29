use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt
{
    #[structopt(name = "url", required = true)]
    origin: String,
}

fn main() {
    let opt = Opt::from_args();
}

