use tokio::process::Command;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .args(["--package", "capi-runtime"])
        .args(["--target", "wasm32-unknown-unknown"])
        .status()
        .await?;

    rocket::build()
        .mount("/", rocket::fs::FileServer::from("."))
        .launch()
        .await?;

    Ok(())
}
