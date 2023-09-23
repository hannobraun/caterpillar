use std::path::PathBuf;

use clap::Parser;

pub fn example() -> anyhow::Result<PathBuf> {
    let args = Args::parse();

    let example_dir = "examples";
    let example = args.example;

    let path = format!("{example_dir}/{example}.cp");
    Ok(PathBuf::from(path))
}

#[derive(clap::Parser)]
struct Args {
    example: String,
}
