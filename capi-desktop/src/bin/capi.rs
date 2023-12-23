use capi_desktop::{args::Args, display, loader::Loader, DesktopThread};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let mut loader = Loader::new();
    let (code, updates) = loader.load(&args.entry_script)?;

    match args.command {
        capi_desktop::args::Command::Run => {
            let desktop_thread =
                DesktopThread::run(args.entry_script, code, updates)?;
            display::start(desktop_thread)?;
        }
        capi_desktop::args::Command::Test => {
            let desktop_thread =
                DesktopThread::test(args.entry_script, code, updates)?;
            desktop_thread.join()?;
        }
    }

    Ok(())
}
