mod platform;

include!(concat!(env!("OUT_DIR"), "/script.rs"));

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    wasm_bindgen_futures::spawn_local(async {
        if let Err(err) = main_inner().await {
            panic!("Error: {err:?}");
        }
    })
}

async fn main_inner() -> anyhow::Result<()> {
    let mut interpreter = capi_core::Interpreter::new(SCRIPT)?;
    let mut context = platform::Context::default();

    interpreter.register_platform([(
        "print",
        platform::print as capi_core::PlatformFunction<platform::Context>,
    )]);

    loop {
        let sleep_duration = match interpreter.step(&mut context)? {
            capi_core::RuntimeState::Running => None,
            capi_core::RuntimeState::Sleeping => context.sleep_duration.take(),
            capi_core::RuntimeState::Finished => break,
        };

        if let Some(sleep_duration) = sleep_duration {
            gloo_timers::future::sleep(sleep_duration).await
        }
    }

    Ok(())
}
