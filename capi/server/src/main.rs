use std::path::PathBuf;

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    println!("Address: {}", args.address);
    println!("Directory to serve: {}", args.serve_dir.display());

    Ok(())
}

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
