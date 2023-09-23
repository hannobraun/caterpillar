use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Args {
    pub script: PathBuf,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
