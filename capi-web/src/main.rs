mod platform;

include!(concat!(env!("OUT_DIR"), "/script.rs"));

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async {
        if let Err(err) = run().await {
            panic!("Error: {err:?}");
        }
    })
}

async fn run() -> anyhow::Result<()> {
    tracing::debug!("Running script:\n{SCRIPT}");

    let mut interpreter = capi_core::Interpreter::new(SCRIPT)?;
    let mut context = platform::Context::default();

    interpreter.register_platform([
        (
            "delay_ms",
            platform::delay_ms
                as capi_core::PlatformFunction<platform::Context>,
        ),
        ("print", platform::print),
    ]);

    loop {
        let sleep_duration = match interpreter.step(&mut context)? {
            capi_core::RuntimeState::Running => None,
            capi_core::RuntimeState::Sleeping => context.sleep_duration.take(),
            capi_core::RuntimeState::Finished => break,
        };

        // Always sleep, even if it's for zero duration, to give the rest of the
        // website a chance to do its thing between steps.
        let sleep_duration =
            sleep_duration.unwrap_or(std::time::Duration::from_millis(0));
        gloo_timers::future::sleep(sleep_duration).await
    }

    Ok(())
}
