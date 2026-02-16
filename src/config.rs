use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    #[arg(long)]
    pub target: String,

    #[arg(long)]
    pub wordlist: String,

    #[arg(long, default_value_t = 100)]
    pub concurrency: usize,
}
