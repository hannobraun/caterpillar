use capi_desktop::{
    args::{Args, Command},
    display, loader, DesktopThread,
};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let Command::Run { script } = args.command;
    let code = loader::load(&script)?;
    let (updates, _watcher) = loader::watch(&script)?;

    let desktop_thread = DesktopThread::run(code, updates)?;
    display::start(desktop_thread)?;

    Ok(())
}
