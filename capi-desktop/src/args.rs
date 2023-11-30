use std::path::PathBuf;

/// Interactive Caterpillar Runtime
#[derive(clap::Parser)]
pub struct Args {
    /// Path of the Caterpillar script
    pub script: PathBuf,

    #[clap(subcommand)]
    pub command: Command,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Run a Caterpillar script
    Run,
}
