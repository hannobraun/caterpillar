#[no_mangle]
pub extern "C" fn on_key(code: u32) {
    log::debug!("{code}");
}
