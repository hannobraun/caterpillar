fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = capi_desktop::args::Args::parse();
    let code = capi_desktop::loader::load::load(&args.script)?;
    let (updates, _watcher) = capi_desktop::loader::watch::watch(&args.script)?;

    let mut interpreter = capi_core::Interpreter::new(&code)?;
    let mut context = capi_desktop::platform::Context {
        pixel_operations: Vec::new(),
    };
    let mut display = None;

    interpreter.register_platform([
        (
            "delay_ms",
            capi_desktop::platform::delay_ms
                as capi_core::PlatformFunction<capi_desktop::platform::Context>,
        ),
        ("pixel_set", capi_desktop::platform::pixel_set),
        ("print", capi_desktop::platform::print),
    ]);

    loop {
        let runtime_state = interpreter.step(&mut context)?;

        for position in context.pixel_operations.drain(..) {
            let mut d = display
                .map(Ok)
                .unwrap_or_else(capi_desktop::display::Display::new)?;

            d.set(position)?;

            display = Some(d);
        }

        let new_code = match runtime_state {
            capi_core::RuntimeState::Running => match updates.try_recv() {
                Ok(new_code) => Some(new_code),
                Err(std::sync::mpsc::TryRecvError::Empty) => None,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
            },
            capi_core::RuntimeState::Sleeping => {
                unreachable!(
                    "No desktop platform functions put runtime to sleep"
                )
            }
            capi_core::RuntimeState::Finished => {
                eprintln!();
                eprintln!("> Program finished.");
                eprintln!("  > will restart on change to script");
                eprintln!("  > press CTRL-C to abort");
                eprintln!();

                match updates.recv() {
                    Ok(new_code) => Some(new_code),
                    Err(std::sync::mpsc::RecvError) => break,
                }
            }
        };

        if let Some(new_code) = new_code {
            interpreter.update(&new_code)?;
        }
    }

    Ok(())
}
