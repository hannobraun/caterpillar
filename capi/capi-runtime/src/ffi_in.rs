use crate::ffi_out::print;

#[no_mangle]
pub extern "C" fn on_init() {
    print("Hello, world!");
}
