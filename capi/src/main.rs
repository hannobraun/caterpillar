use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    time::Duration,
};

use futures::Stream;
use http::StatusCode;
use notify::RecommendedWatcher;
use notify_debouncer_mini::{
    DebounceEventResult, DebouncedEventKind, Debouncer,
};
use tokio::{
    fs,
    sync::watch::{self, Receiver, Sender},
    task::{self, JoinHandle},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let serve_dir = tempfile::tempdir()?;
    let path = serve_dir.path().to_owned();

    let (_watcher, watch_events) = watcher()?;
    let (builder, build_events) = builder(watch_events, path.clone());
    let server = server(path, build_events);

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
) -> (JoinHandle<anyhow::Result<()>>, Receiver<()>) {
    let (tx, rx) = watch::channel(());
    let builder = task::spawn(build(watch_events, tx, serve_dir));
    (builder, rx)
}

async fn build(
    mut watch_events: Receiver<()>,
    build_events: Sender<()>,
    serve_dir: PathBuf,
) -> anyhow::Result<()> {
    while let Ok(()) = watch_events.changed().await {
        println!("Change detected. Building...");

        fs::copy("index.html", serve_dir.join("index.html")).await?;

        build_events.send(())?;
    }

    Ok(())
}

fn server(
    serve_dir: PathBuf,
    events: Receiver<()>,
) -> JoinHandle<anyhow::Result<()>> {
    task::spawn(serve(serve_dir, events))
}

async fn serve(serve_dir: PathBuf, events: Receiver<()>) -> anyhow::Result<()> {
    use warp::Filter;

    let address: SocketAddr = ([127, 0, 0, 1], 8080).into();
    println!("Starting server at http://{address}");

    warp::serve(
        warp::get().and(
            warp::path("update")
                .and(warp::body::stream())
                .map(move |stream| {
                    let mut events = events.clone();
                    events.mark_unchanged();
                    (stream, events)
                })
                .then(update)
                .or(warp::fs::dir(serve_dir)),
        ),
    )
    .run(address)
    .await;

    Ok(())
}

async fn update((_, mut events): (impl Stream, Receiver<()>)) -> StatusCode {
    if events.changed().await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}
