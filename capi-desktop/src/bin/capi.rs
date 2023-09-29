fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = capi_desktop::args::Args::parse();
    let code = capi_desktop::loader::load::load(&args.script)?;
    let (updates, _watcher) = capi_desktop::loader::watch::watch(&args.script)?;

    let pixel_ops = capi_desktop::thread::run(code, updates)?;

    let mut display = None;
    for capi_desktop::platform::PixelOp::Set(position) in pixel_ops.iter() {
        let mut d = display
            .map(Ok)
            .unwrap_or_else(capi_desktop::display::Display::new)?;

        d.set(position)?;

        display = Some(d);
    }

    Ok(())
}
