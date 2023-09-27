mod platform;
mod ui;

include!(concat!(env!("OUT_DIR"), "/script.rs"));

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let output = sycamore::reactive::create_rc_signal(String::new());
    let output2 = output.clone();

    wasm_bindgen_futures::spawn_local(async {
        if let Err(err) = platform::run(SCRIPT, output2).await {
            panic!("Error: {err:?}");
        }
    });
    ui::render(output);
}
