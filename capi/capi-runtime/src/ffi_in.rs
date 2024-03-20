extern "C" {
    fn ffi_print(ptr: *const u8, len: usize);
}

#[no_mangle]
pub extern "C" fn on_init() {
    let s = "Hello, world!";
    unsafe { ffi_print(s.as_ptr(), s.len()) };
}
