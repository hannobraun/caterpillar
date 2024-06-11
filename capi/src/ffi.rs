#[no_mangle]
pub extern "C" fn on_key(code: u8) {
    log::debug!("{code}");
}
