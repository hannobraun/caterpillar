use std::path::PathBuf;

pub fn example() -> PathBuf {
    let args = Args::parse();
    args.example
}

#[derive(clap::Parser)]
pub struct Args {
    pub example: PathBuf,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
