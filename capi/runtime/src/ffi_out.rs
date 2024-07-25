pub fn on_panic(message: &str) {
    unsafe {
        ffi::on_panic(message.as_ptr(), message.len());
    }
}

mod ffi {
    extern "C" {
        pub fn on_panic(ptr: *const u8, len: usize);
    }
}
