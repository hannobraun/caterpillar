use tempfile::{tempdir, TempDir};
use tokio::{fs, process::Command};

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let path = build().await?;
    serve(path).await?;

    Ok(())
}

async fn build() -> anyhow::Result<TempDir> {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .args(["--package", "capi-runtime"])
        .args(["--target", "wasm32-unknown-unknown"])
        .status()
        .await?;

    let dir = tempdir()?;
    fs::copy("index.html", dir.path().join("index.html")).await?;
    fs::copy(
        "target/wasm32-unknown-unknown/release/capi_runtime.wasm",
        dir.path().join("capi_runtime.wasm"),
    )
    .await?;

    Ok(dir)
}

async fn serve(path: TempDir) -> anyhow::Result<()> {
    rocket::build()
        .mount("/", rocket::fs::FileServer::from(&path))
        .launch()
        .await?;

    Ok(())
}
