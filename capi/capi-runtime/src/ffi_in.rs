use crate::ffi_out::console_log;

#[no_mangle]
pub extern "C" fn on_init() {
    console_log("Hello, world!");
}
