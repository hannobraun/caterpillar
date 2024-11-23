mod args;
mod build_game;
mod files;
mod headless;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::path::PathBuf;

    use self::args::Args;

    tracing_subscriber::fmt().init();

    let args = Args::parse();

    match args {
        Args::Headless => {
            headless::run().await?;
        }
        Args::Serve { address, serve_dir } => {
            let files = files::FILES;

            if !files.list_invalid().is_empty() {
                dbg!(files.list_invalid());
            }

            let mut events =
                server::start(PathBuf::from("games"), address, serve_dir)
                    .await?;

            while let Some(event) = events.recv().await {
                match event {
                    server::Event::ChangeDetected => {
                        print!(
                            "\n\
                            ⏳ Change detected. Building game...\n"
                        );
                    }
                    server::Event::BuildFinished => {
                        println!("✅ Finished building game.");
                    }
                    server::Event::ServerReady => {
                        print!(
                            "\n\
                            🚀 Build is ready: http://{address}/ 🚀\n\
                            \n"
                        );
                    }
                }
            }

            tracing::info!("`capi-server` shutting down.");
        }
    }

    Ok(())
}
