use std::path::PathBuf;

/// Caterpillar server
#[derive(clap::Parser)]
pub struct Args {
    /// Address to serve at
    #[arg(short, long)]
    pub address: String,

    /// Directory to serve from
    #[arg(short, long)]
    pub serve_dir: PathBuf,
}
