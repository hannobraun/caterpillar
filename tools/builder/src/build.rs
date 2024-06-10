use std::process;

use tokio::{fs, process::Command, sync::watch, task};
use tracing::error;
use wasm_bindgen_cli_support::Bindgen;

use crate::watch::DebouncedChanges;

pub fn start(changes: DebouncedChanges) -> UpdatesRx {
    let (tx, rx) = watch::channel(());
    task::spawn(async {
        if let Err(err) = watch_and_build(changes, tx).await {
            error!("Build error: {err}");
            process::exit(1);
        }
    });
    rx
}

async fn watch_and_build(
    mut changes: DebouncedChanges,
    updates: watch::Sender<()>,
) -> anyhow::Result<()> {
    build_once(&updates).await?;

    while changes.wait_for_change().await {
        if !build_once(&updates).await? {
            break;
        }
    }

    Ok(())
}

async fn build_once(updates: &watch::Sender<()>) -> anyhow::Result<bool> {
    let cargo_build = Command::new("cargo")
        .arg("build")
        .args(["--package", "capi"])
        .args(["--target", "wasm32-unknown-unknown"])
        .status()
        .await?;
    if !cargo_build.success() {
        return Ok(true);
    }

    let mut bindgen = Bindgen::new();
    bindgen
        .input_path("target/wasm32-unknown-unknown/debug/capi.wasm")
        .web(true)?
        .generate("capi/dist")?;

    fs::copy("capi/index.html", "capi/dist/index.html").await?;

    if updates.send(()).is_err() {
        return Ok(false);
    }

    Ok(true)
}

pub type UpdatesRx = watch::Receiver<()>;
