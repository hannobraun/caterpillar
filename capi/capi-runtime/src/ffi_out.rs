#[cfg_attr(not(target_arch = "wasm32"), allow(unused))]
pub fn console_error(s: &str) {
    unsafe { ffi::console_error(s.as_ptr(), s.len()) }
}

pub fn console_log(s: &str) {
    unsafe { ffi::console_log(s.as_ptr(), s.len()) };
}

mod ffi {
    extern "C" {
        pub fn console_error(ptr: *const u8, len: usize);
        pub fn console_log(ptr: *const u8, len: usize);
    }
}
