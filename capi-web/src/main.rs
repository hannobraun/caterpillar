fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    if let Err(err) = main_inner() {
        panic!("Error: {err}")
    }
}

fn main_inner() -> anyhow::Result<()> {
    Ok(())
}
