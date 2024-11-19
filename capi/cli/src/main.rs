mod args;
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
            let mut events =
                server::start(PathBuf::from("games"), address, serve_dir)
                    .await?;

            while let Some(event) = events.recv().await {
                match event {
                    server::Event::ChangeDetected => {
                        println!("build:change");
                    }
                    server::Event::BuildFinished => {
                        println!("build:finish");
                    }
                    server::Event::ServerReady => {
                        println!("ready");
                    }
                }
            }

            tracing::info!("`capi-server` shutting down.");
        }
    }

    Ok(())
}
