use std::path::PathBuf;

/// Interactive Caterpillar Runtime
#[derive(clap::Parser)]
pub struct Args {
    /// Path to a Caterpillar script
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
    /// Run the Caterpillar script
    Run,

    /// Run all tests from the Caterpillar script
    Test,
}
