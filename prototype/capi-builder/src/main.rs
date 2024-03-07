use std::path::Path;

use tempfile::tempdir;
use tokio::{fs, process::Command};

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let serve_dir = tempdir()?;

    build(&serve_dir).await?;
    serve(&serve_dir).await?;

    Ok(())
}

async fn build(serve_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let serve_dir = serve_dir.as_ref();

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

    Ok(())
}

async fn serve(serve_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    rocket::build()
        .mount("/", rocket::fs::FileServer::from(&serve_dir))
        .launch()
        .await?;

    Ok(())
}
