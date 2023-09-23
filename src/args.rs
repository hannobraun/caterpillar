use std::path::PathBuf;

use clap::Parser;

pub fn example() -> anyhow::Result<PathBuf> {
    let args = Args::parse();
    Ok(PathBuf::from(args.example))
}

#[derive(clap::Parser)]
struct Args {
    example: String,
}
