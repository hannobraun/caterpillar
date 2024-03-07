extern "C" {
    fn print_ffi(ptr: *const u8, len: usize);
}

pub fn print(s: &str) {
    unsafe {
        print_ffi(s.as_ptr(), s.len());
    }
}
