use std::{fs, path::PathBuf};

use anyhow::bail;
use clap::Parser;

pub fn example() -> anyhow::Result<PathBuf> {
    let args = Args::parse();

    let example_dir = "examples";
    if let Some(example) = args.example {
        let path = format!("{example_dir}/{example}.cp");
        return Ok(PathBuf::from(path));
    }

    eprintln!("Need to specify example. Available examples:");

    for dir_entry in fs::read_dir(example_dir)? {
        let path = dir_entry?.path();
        let example = path.file_stem().unwrap().to_string_lossy();
        eprintln!("- {example}");
    }

    bail!("No example specified")
}

#[derive(clap::Parser)]
struct Args {
    example: Option<String>,
}
