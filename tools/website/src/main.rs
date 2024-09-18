use std::{fs, path::Path, process::Command};

use anyhow::anyhow;
use tempfile::TempDir;
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    let source_dir = Path::new("website");
    let target_dir = TempDir::new()?;

    for entry in WalkDir::new(source_dir) {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }

        let source_path = entry.path();
        let target_path = {
            let path_within_source_dir =
                entry.path().strip_prefix(source_dir)?;
            target_dir.path().join(path_within_source_dir)
        };

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source_path, target_path)?;
    }

    serve(target_dir.path())?;

    Ok(())
}

fn serve(path: &Path) -> anyhow::Result<()> {
    let exit_status = Command::new("deno")
        .args(["task", "start"])
        .current_dir(path)
        .status()?;

    if !exit_status.success() {
        return Err(anyhow!(
            "`deno task start` return exit code `{:?}`",
            exit_status.code()
        ));
    }

    Ok(())
}
