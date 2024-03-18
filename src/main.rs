use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use notify_debouncer_mini::{DebounceEventResult, DebouncedEventKind};
use tokio::{
    fs,
    sync::watch::{self, Receiver},
    task,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let serve_dir = tempfile::tempdir()?;

    let events = watch_files()?;
    task::spawn(build_on_changes(events));

    fs::copy("index.html", serve_dir.path().join("index.html")).await?;

    serve_build(serve_dir.path().to_owned()).await;

    Ok(())
}

fn watch_files() -> anyhow::Result<Receiver<()>> {
    let (tx, rx) = watch::channel(());

    let mut debouncer = notify_debouncer_mini::new_debouncer(
        Duration::from_millis(50),
        move |result: DebounceEventResult| {
            let events = result.expect("Error watching for changes");

            for event in events {
                if let DebouncedEventKind::Any = event.kind {
                    // Should only panic, if the other end panicked, causing the
                    // receiver to drop. Nothing we can do about it.
                    let _ = tx.send(());
                }
            }
        },
    )?;
    debouncer
        .watcher()
        .watch(Path::new("index.html"), notify::RecursiveMode::NonRecursive)?;

    Ok(rx)
}

async fn build_on_changes(mut events: Receiver<()>) {
    while let Ok(()) = events.changed().await {
        println!("Change detected.");
    }
}

async fn serve_build(serve_dir: PathBuf) {
    warp::serve(warp::fs::dir(serve_dir))
        .run(([127, 0, 0, 1], 8080))
        .await;
}
