use std::{fs, path::PathBuf};

use anyhow::bail;
use clap::Parser;

pub fn example() -> anyhow::Result<PathBuf> {
    let args = Args::parse();

    let example_dir = "cp7/examples";
    let path = if let Some(example) = args.example {
        format!("cp7/examples/{example}.cp")
    } else {
        eprintln!("Need to specify example. Available examples:");

        for dir_entry in fs::read_dir(example_dir)? {
            let path = dir_entry?.path();
            let example = path.file_stem().unwrap().to_string_lossy();
            eprintln!("- {example}");
        }

        bail!("No example specified")
    };

    Ok(PathBuf::from(path))
}

#[derive(clap::Parser)]
pub struct Args {
    pub example: Option<String>,
}
