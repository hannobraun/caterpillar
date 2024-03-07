use std::{path::Path, time::Duration};

use notify_debouncer_mini::{
    new_debouncer,
    notify::{RecommendedWatcher, RecursiveMode},
    DebounceEventResult, DebouncedEventKind, Debouncer,
};
use tempfile::tempdir;
use tokio::{
    fs,
    process::Command,
    sync::watch,
    task::{self},
};

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let serve_dir = tempdir()?;

    let _watcher = watch(&serve_dir)?;
    serve(&serve_dir).await?;

    Ok(())
}

fn watch(
    serve_dir: impl AsRef<Path>,
) -> anyhow::Result<Debouncer<RecommendedWatcher>> {
    let (tx, rx) = watch::channel(());

    let mut debouncer = new_debouncer(
        Duration::from_millis(50),
        move |result: DebounceEventResult| {
            let events = result.unwrap();
            for event in events {
                if event.kind == DebouncedEventKind::Any {
                    tx.send(()).unwrap();
                }
            }
        },
    )?;
    debouncer
        .watcher()
        .watch(Path::new("capi-runtime"), RecursiveMode::Recursive)?;

    let serve_dir = serve_dir.as_ref().to_path_buf();
    task::spawn(build(serve_dir, rx));

    Ok(debouncer)
}

async fn build(
    serve_dir: impl AsRef<Path>,
    mut changes: watch::Receiver<()>,
) -> anyhow::Result<()> {
    let serve_dir = serve_dir.as_ref();

    loop {
        let () = changes.changed().await.unwrap();

        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .args(["--package", "capi-runtime"])
            .args(["--target", "wasm32-unknown-unknown"])
            .status()
            .await?;

        fs::copy("index.html", serve_dir.join("index.html")).await?;
        fs::copy(
            "target/wasm32-unknown-unknown/release/capi_runtime.wasm",
            serve_dir.join("capi_runtime.wasm"),
        )
        .await?;
    }
}

async fn serve(serve_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    rocket::build()
        .mount("/", rocket::fs::FileServer::from(&serve_dir))
        .launch()
        .await?;

    Ok(())
}
