use std::path::PathBuf;

/// Interactive Caterpillar Runtime
#[derive(clap::Parser)]
pub struct Args {
    /// path of the Caterpillar script
    pub script: PathBuf,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
