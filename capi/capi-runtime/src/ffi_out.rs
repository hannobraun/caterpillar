extern "C" {
    fn ffi_print(ptr: *const u8, len: usize);
}

pub fn print(s: &str) {
    unsafe { ffi_print(s.as_ptr(), s.len()) };
}
