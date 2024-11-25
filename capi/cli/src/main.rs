mod args;
mod build_game;
mod files;
mod headless;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::{fmt::Write, path::PathBuf};

    use anyhow::anyhow;

    use self::args::Args;

    tracing_subscriber::fmt().init();

    let args = Args::parse();

    match args {
        Args::Headless => {
            headless::run().await?;
        }
        Args::Serve { address } => {
            let files = files::FILES;

            if !files.list_invalid().is_empty() {
                let mut err = String::new();

                write!(
                    err,
                    "Can't start the server because the \
                    following files are not available:\n\
                    \n",
                )?;

                for file in files.list_invalid() {
                    writeln!(err, "- `{file}`")?;
                }

                write!(
                    err,
                    "\n\
                    All of those files should be included\n\
                    with this tool! That they aren't, means\n\
                    that the tool has not been built\n\
                    correctly.\n\
                    \n\
                    Are you trying to run the server from\n\
                    within the Caterpillar repository?\n\
                    \n\
                    \tThen do so through the build tool!\n\
                    \tJust execute `cargo run` from the\n\
                    \trepository root.\n\
                    \n\
                    Are you trying to build a version of\n\
                    this tool for use outside of the\n\
                    repository?\n\
                    \n\
                    \tSorry, but as of 2024-11-23, this is\n\
                    \tnot supported yet! If you're reading\n\
                    \tthis some time after that date, then\n\
                    \tmaybe it has become possible in the\n\
                    \tmeantime, and this error message has not\n\
                    \tbeen updated.\n",
                )?;

                return Err(anyhow!("{}", err));
            }

            let mut events =
                server::start(PathBuf::from("games"), address).await?;

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
