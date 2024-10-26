mod server;

pub async fn start(
    address: String,
    serve_dir: std::path::PathBuf,
    game: tokio::sync::watch::Receiver<
        capi_protocol::Versioned<capi_build_game::CompilerOutput>,
    >,
) -> anyhow::Result<()> {
    server::start(address, serve_dir, game).await?;
    Ok(())
}
