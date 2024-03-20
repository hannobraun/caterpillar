extern "C" {
    fn ffi_print(ptr: *const u8, len: usize);
}

pub fn print(s: &str) {
    unsafe { ffi_print(s.as_ptr(), s.len()) };
}

#[no_mangle]
pub extern "C" fn on_init() {
    let s = "Hello, world!";
    print(s);
}
