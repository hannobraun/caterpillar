fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = capi_desktop::args::Args::parse();
    let code = capi_desktop::loader::load::load(&args.script)?;
    let (updates, _watcher) = capi_desktop::loader::watch::watch(&args.script)?;

    let desktop_thread = capi_desktop::DesktopThread::run(code, updates)?;
    let desktop_thread = capi_desktop::display::start(desktop_thread)?;

    // If we reach this point, then the main thread returned from the graphics
    // subsystem. This must mean the Caterpillar thread ended.
    desktop_thread.join()?;

    Ok(())
}
