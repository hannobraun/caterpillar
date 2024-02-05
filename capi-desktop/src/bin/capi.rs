use anyhow::Context;
use capi_desktop::{args::Args, display, DesktopThread};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    match args.command {
        capi_desktop::args::Command::Run => {
            let desktop_thread = DesktopThread::run(args.entry_script)
                .context("Failed to set up desktop thread for running")?;
            display::start(desktop_thread)
                .context("Failed to start display")?;
        }
        capi_desktop::args::Command::Test => {
            let desktop_thread = DesktopThread::test(args.entry_script)
                .context("Failed to set up desktop thread for testing")?;
            desktop_thread
                .join()
                .context("Error while running desktop thread")?;
        }
    }

    Ok(())
}
