use std::path::Path;

use tokio::process::Command;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    build().await?;
    serve(".").await?;

    Ok(())
}

async fn build() -> anyhow::Result<()> {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .args(["--package", "capi-runtime"])
        .args(["--target", "wasm32-unknown-unknown"])
        .status()
        .await?;

    Ok(())
}

async fn serve(path: impl AsRef<Path>) -> anyhow::Result<()> {
    rocket::build()
        .mount("/", rocket::fs::FileServer::from(path))
        .launch()
        .await?;

    Ok(())
}
