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
    interpreter.register_platform([(
        "print",
        platform::print as capi_core::PlatformFunction<()>,
    )]);

    loop {
        match interpreter.step(&mut ())? {
            capi_core::RuntimeState::Running => {}
            capi_core::RuntimeState::Sleeping => {
                unreachable!("No web platform functions put runtime to sleep")
            }
            capi_core::RuntimeState::Finished => break,
        }
    }

    Ok(())
}
