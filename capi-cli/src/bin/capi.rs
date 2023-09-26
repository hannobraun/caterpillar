fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = capi_cli::args::Args::parse();
    let code = capi_cli::loader::load::load(&args.script)?;
    let (updates, _watcher) = capi_cli::loader::watch::watch(&args.script)?;

    let mut interpreter = capi_core::Interpreter::new(&code)?;

    interpreter.register_platform([
        (
            "print",
            capi_cli::functions::print as capi_core::NativeFunction,
        ),
        ("delay_ms", capi_cli::functions::delay_ms),
    ]);

    loop {
        let new_code = match interpreter.step()? {
            capi_core::EvaluatorState::InProgress => match updates.try_recv() {
                Ok(new_code) => Some(new_code),
                Err(std::sync::mpsc::TryRecvError::Empty) => None,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
            },
            capi_core::EvaluatorState::Finished => {
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
