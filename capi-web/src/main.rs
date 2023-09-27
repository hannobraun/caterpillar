include!(concat!(env!("OUT_DIR"), "/script.rs"));

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    if let Err(err) = main_inner() {
        panic!("Error: {err:?}");
    }
}

fn main_inner() -> anyhow::Result<()> {
    let mut interpreter = capi_core::Interpreter::new(SCRIPT)?;
    interpreter.register_platform([(
        "print",
        print as capi_core::PlatformFunction<()>,
    )]);

    while !interpreter.step(&mut ())?.finished() {}

    Ok(())
}

fn print(
    context: capi_core::RuntimeContext,
    _: &mut (),
) -> capi_core::DataStackResult<capi_core::FunctionState> {
    let value = context.data_stack.pop_any()?;
    tracing::info!("{}", value.kind);
    context.data_stack.push(value);
    Ok(capi_core::FunctionState::Resume)
}
