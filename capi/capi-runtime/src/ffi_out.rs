pub fn print(s: &str) {
    unsafe { ffi::console_log(s.as_ptr(), s.len()) };
}

mod ffi {
    extern "C" {
        pub fn console_log(ptr: *const u8, len: usize);
    }
}
