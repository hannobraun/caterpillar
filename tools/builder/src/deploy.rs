use tokio::process::Command;

use crate::build::build_once;

pub async fn run() -> anyhow::Result<()> {
    let files = build_once().await?;

    if let Some(files) = files {
        Command::new("cargo")
            .arg("run")
            .args(["--package", "capi-cli"])
            .arg("--")
            .arg("export")
            .args(["--path", "deployment"])
            .env("FILES", files.path().display().to_string())
            .kill_on_drop(true)
            .status()
            .await?;
    }

    Ok(())
}
