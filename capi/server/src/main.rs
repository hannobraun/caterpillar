mod args;
mod build;
mod server;

use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = args::Args::parse();

    let (source_code, bytecode) = build::build_snake().await?;

    server::start(args.address, args.serve_dir, source_code, bytecode).await?;

    info!("`capi-server` shutting down.");
    Ok(())
}
