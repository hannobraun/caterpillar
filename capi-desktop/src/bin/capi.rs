use capi_desktop::{args::Args, display, loader, DesktopThread};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let code = loader::load(&args.script)?;
    let (updates, _watcher) = loader::watch(&args.script)?;

    let desktop_thread = DesktopThread::run(code, updates)?;
    display::start(desktop_thread)?;

    Ok(())
}
