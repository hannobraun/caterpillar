use std::{
    io,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Context;
use capi_assembler::assemble;
use notify_debouncer_mini::{
    new_debouncer,
    notify::{RecommendedWatcher, RecursiveMode},
    DebounceEventResult, DebouncedEventKind, Debouncer,
};
use rocket::State;
use tempfile::tempdir;
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
    sync::watch,
    task,
};

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let serve_dir = tempdir()?;

    let _host_watcher = watch_host(serve_dir.path().to_path_buf())?;
    let _runtime_watcher = watch_runtime(serve_dir.path().to_path_buf())?;
    let _snake_watcher =
        watch_assembler(serve_dir.path().to_path_buf()).await?;
    serve(serve_dir.path().to_path_buf()).await?;

    Ok(())
}

fn watch_host(
    serve_dir: PathBuf,
) -> anyhow::Result<Debouncer<RecommendedWatcher>> {
    let (tx, rx) = watch::channel(());
    tx.send_replace(());

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
        .watch(Path::new("index.html"), RecursiveMode::Recursive)?;

    task::spawn(copy_host(serve_dir, rx));

    Ok(debouncer)
}

async fn copy_host(
    serve_dir: PathBuf,
    mut changes: watch::Receiver<()>,
) -> anyhow::Result<()> {
    loop {
        let () = changes.changed().await.unwrap();

        // Remove file before the build, to prevent anybody from loading a stale
        // stale version after a change.
        let _ =
            fs::remove_file(serve_dir.join(serve_dir.join("index.html"))).await;

        println!("Copying host...");
        copy_artifacts(&serve_dir).await?;
    }
}

fn watch_runtime(
    serve_dir: PathBuf,
) -> anyhow::Result<Debouncer<RecommendedWatcher>> {
    let (tx, rx) = watch::channel(());
    tx.send_replace(());

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

    // Here we're watching `capi-runtime` and its dependencies. It would be more
    // robust to figure out those dependencies automatically.
    debouncer
        .watcher()
        .watch(Path::new("capi-runtime"), RecursiveMode::Recursive)?;
    debouncer
        .watcher()
        .watch(Path::new("capi-vm"), RecursiveMode::Recursive)?;

    task::spawn(build_runtime(serve_dir, rx));

    Ok(debouncer)
}

async fn build_runtime(
    serve_dir: impl AsRef<Path>,
    mut changes: watch::Receiver<()>,
) -> anyhow::Result<()> {
    let serve_dir = serve_dir.as_ref();

    loop {
        let () = changes.changed().await.unwrap();

        // Remove file before the build, to prevent anybody from loading a stale
        // stale version after a change.
        let _ = fs::remove_file(
            serve_dir.join(serve_dir.join("capi_runtime.wasm")),
        )
        .await;

        let exit_status = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .args(["--package", "capi-runtime"])
            .args(["--target", "wasm32-unknown-unknown"])
            .status()
            .await?;

        if exit_status.success() {
            copy_artifacts(serve_dir).await?;
        }
    }
}

async fn watch_assembler(
    serve_dir: PathBuf,
) -> anyhow::Result<Debouncer<RecommendedWatcher>> {
    let (tx, rx) = watch::channel(());
    tx.send_replace(());

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
        .watch(Path::new("snake.asm.capi"), RecursiveMode::Recursive)?;

    task::spawn(build_snake(serve_dir, rx));

    Ok(debouncer)
}

async fn build_snake(
    serve_dir: impl AsRef<Path>,
    mut changes: watch::Receiver<()>,
) -> anyhow::Result<()> {
    let serve_dir = serve_dir.as_ref();

    loop {
        let () = changes.changed().await.unwrap();

        // Remove file before the build, to prevent anybody from loading a stale
        // stale version after a change.
        let _ =
            fs::remove_file(serve_dir.join(serve_dir.join("snake.bc.capi")))
                .await;

        let mut assembly = String::new();
        File::open("snake.asm.capi")
            .await
            .context("Opening assembly")?
            .read_to_string(&mut assembly)
            .await
            .context("Reading assembly")?;

        let bytecode = match assemble(&assembly) {
            Ok(bytecode) => bytecode,
            Err(err) => {
                println!("Assembly error: {err}");
                continue;
            }
        };

        File::create("snake.bc.capi")
            .await?
            .write_all(&bytecode)
            .await?;

        copy_artifacts(serve_dir).await?;
    }
}

async fn copy_artifacts(serve_dir: &Path) -> anyhow::Result<()> {
    fs::copy("index.html", serve_dir.join("index.html")).await?;
    fs::copy(
        "target/wasm32-unknown-unknown/release/capi_runtime.wasm",
        serve_dir.join("capi_runtime.wasm"),
    )
    .await?;
    fs::copy("snake.bc.capi", serve_dir.join("snake.bc.capi")).await?;

    Ok(())
}

async fn serve(serve_dir: PathBuf) -> anyhow::Result<()> {
    rocket::build()
        .manage(serve_dir.clone())
        .mount("/", rocket::fs::FileServer::from(&serve_dir))
        .mount("/", rocket::routes![code])
        .launch()
        .await?;

    Ok(())
}

#[rocket::get("/code")]
async fn code(serve_dir: &State<PathBuf>) -> io::Result<File> {
    let file = File::open(serve_dir.join("snake.bc.capi")).await?;
    Ok(file)
}
