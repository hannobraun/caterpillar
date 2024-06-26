pub use capi_debugger::ffi;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Error)
        .expect("Failed to initialize logging to console");
}
