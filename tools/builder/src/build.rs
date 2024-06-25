use std::{
    path::{Path, PathBuf},
    process,
};

use tempfile::{tempdir, TempDir};
use tokio::{fs, process::Command, sync::watch, task};
use tracing::error;
use wasm_bindgen_cli_support::Bindgen;

use crate::watch::DebouncedChanges;

pub fn start(changes: DebouncedChanges) -> UpdatesRx {
    let (tx, rx) = watch::channel(None);
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
    updates: UpdatesTx,
) -> anyhow::Result<()> {
    println!();
    println!("Starting initial build of Caterpillar...");
    println!();

    // We're not really doing anything with this variable, but it needs to
    // exist. It keeps the `TempDir` instances from being dropped before we're
    // done with it. Dropping it prematurely would delete the temporary
    // directory we serve files out of.
    let mut output_dir = None;

    build_once(&updates, &mut output_dir).await?;

    while changes.wait_for_change().await {
        println!();
        println!("Change detected. Rebuilding Caterpillar...");
        println!();

        let should_continue = build_once(&updates, &mut output_dir).await?;
        if let ShouldContinue::NoBecauseShutdown = should_continue {
            break;
        }
    }

    Ok(())
}

async fn build_once(
    updates: &UpdatesTx,
    serve_dir: &mut Option<TempDir>,
) -> anyhow::Result<ShouldContinue> {
    let cargo_build = Command::new("cargo")
        .arg("build")
        .args(["--package", "capi-runtime"])
        .args(["--target", "wasm32-unknown-unknown"])
        .status()
        .await?;
    if !cargo_build.success() {
        // The build failed, and since the rest of this function is dependent on
        // its success, we're done here.
        //
        // But that doesn't mean that the builder overall should be done. Next
        // time we detect a change, we should try again.
        return Ok(ShouldContinue::YesWhyNot);
    }

    let crate_to_serve = Path::new("capi/runtime");
    let new_serve_dir = tempdir()?;

    let mut bindgen = Bindgen::new();
    bindgen
        .input_path("target/wasm32-unknown-unknown/debug/capi-runtime.wasm")
        .web(true)?
        .generate(&new_serve_dir)?;

    fs::copy(
        crate_to_serve.join("index.html"),
        new_serve_dir.path().join("index.html"),
    )
    .await?;

    if updates
        .send(Some(new_serve_dir.path().to_path_buf()))
        .is_err()
    {
        // If the send failed, the other end has hung up. That means either
        // we're currently shutting down, or something went wrong over there and
        // we _should_ be shutting down.
        return Ok(ShouldContinue::NoBecauseShutdown);
    }

    *serve_dir = Some(new_serve_dir);

    Ok(ShouldContinue::YesWhyNot)
}

enum ShouldContinue {
    YesWhyNot,
    NoBecauseShutdown,
}

pub type UpdatesRx = watch::Receiver<Update>;
pub type UpdatesTx = watch::Sender<Update>;

pub type Update = Option<PathBuf>;
