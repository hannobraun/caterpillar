use std::{fs, path::Path, process::Command};

use anyhow::anyhow;
use clap::Parser;
use tempfile::TempDir;
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let source_dir = Path::new("website");

    if args.build {
        let target_dir = Path::new("website-output");

        build(source_dir, target_dir)?;
    } else {
        let target_dir = TempDir::new()?;

        build(source_dir, target_dir.path())?;
        serve(target_dir.path())?;
    }

    Ok(())
}

fn build(source_dir: &Path, target_dir: &Path) -> anyhow::Result<()> {
    for entry in WalkDir::new(source_dir) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }

        let source_path = entry.path();
        let target_path = {
            let path_within_source_dir =
                entry.path().strip_prefix(source_dir)?;
            target_dir.join(path_within_source_dir)
        };

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source_path, target_path)?;
    }

    Ok(())
}

fn serve(path: &Path) -> anyhow::Result<()> {
    let exit_status = Command::new("deno")
        .args(["task", "start"])
        .current_dir(path)
        .status()?;

    if !exit_status.success() {
        return Err(anyhow!(
            "`deno task start` returned exit code `{:?}`",
            exit_status.code()
        ));
    }

    Ok(())
}

#[derive(clap::Parser)]
pub struct Args {
    #[arg(short, long)]
    pub build: bool,
}
