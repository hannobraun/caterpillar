pub fn print(s: &str) {
    unsafe { ffi::ffi_print(s.as_ptr(), s.len()) };
}

mod ffi {
    extern "C" {
        pub fn ffi_print(ptr: *const u8, len: usize);
    }
}
