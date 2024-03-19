use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    time::Duration,
};

use futures::Stream;
use notify::RecommendedWatcher;
use notify_debouncer_mini::{
    DebounceEventResult, DebouncedEventKind, Debouncer,
};
use tokio::{
    fs,
    sync::watch::{self, Receiver},
    task::{self, JoinHandle},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let serve_dir = tempfile::tempdir()?;
    let path = serve_dir.path().to_owned();

    let (_watcher, watch_events) = watcher()?;
    let builder = builder(watch_events, path.clone());
    let server = server(path);

    tokio::select! {
        result = builder => { result??; }
        result = server => { result??; }
    }

    Ok(())
}

fn watcher() -> anyhow::Result<(Debouncer<RecommendedWatcher>, Receiver<()>)> {
    let (tx, mut rx) = watch::channel(());
    rx.mark_changed();

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

    Ok((debouncer, rx))
}

fn builder(
    watch_events: Receiver<()>,
    serve_dir: PathBuf,
) -> JoinHandle<anyhow::Result<()>> {
    task::spawn(build(watch_events, serve_dir))
}

async fn build(
    mut events: Receiver<()>,
    serve_dir: PathBuf,
) -> anyhow::Result<()> {
    while let Ok(()) = events.changed().await {
        println!("Change detected. Building...");

        fs::copy("index.html", serve_dir.join("index.html")).await?;
    }

    Ok(())
}

fn server(serve_dir: PathBuf) -> JoinHandle<anyhow::Result<()>> {
    task::spawn(serve(serve_dir))
}

async fn serve(serve_dir: PathBuf) -> anyhow::Result<()> {
    use warp::Filter;

    let address: SocketAddr = ([127, 0, 0, 1], 8080).into();
    println!("Starting server at http://{address}");

    warp::serve(
        warp::get().and(
            warp::path("update")
                .and(warp::body::stream())
                .then(update)
                .or(warp::fs::dir(serve_dir)),
        ),
    )
    .run(address)
    .await;

    Ok(())
}

async fn update(_: impl Stream) -> &'static str {
    "Hello, world!"
}
