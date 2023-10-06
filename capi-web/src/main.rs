mod platform;
mod ui;

include!(concat!(env!("OUT_DIR"), "/script.rs"));

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let (_, code_rx) = async_channel::unbounded();
    let (output_tx, output_rx) = async_channel::unbounded();

    wasm_bindgen_futures::spawn_local(async {
        if let Err(err) = platform::run(SCRIPT, code_rx, output_tx).await {
            panic!("Platform error: {err:?}");
        }
    });
    wasm_bindgen_futures::spawn_local(async {
        if let Err(err) = ui::render(SCRIPT, output_rx).await {
            panic!("UI error: {err:?}");
        }
    });
}
