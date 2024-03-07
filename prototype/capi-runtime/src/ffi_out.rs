extern "C" {
    fn print_ffi(ptr: *const u8, len: usize);
    fn random_ffi() -> f32;
}

pub fn print(s: &str) {
    unsafe {
        print_ffi(s.as_ptr(), s.len());
    }
}

pub fn random() -> f32 {
    unsafe { random_ffi() }
}
