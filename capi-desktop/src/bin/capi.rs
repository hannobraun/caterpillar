fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = capi_desktop::args::Args::parse();
    let code = capi_desktop::loader::load::load(&args.script)?;
    let (updates, _watcher) = capi_desktop::loader::watch::watch(&args.script)?;

    capi_desktop::thread::run(code, updates)?;

    Ok(())
}
