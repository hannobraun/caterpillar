use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
    pub example: PathBuf,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
