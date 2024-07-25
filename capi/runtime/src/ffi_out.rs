pub fn on_panic(message: &str) {
    // Sound, as the `on_panic` function immediately builds and logs a
    // JavaScript string, and the pointer it not kept around after that.
    unsafe {
        ffi::on_panic(message.as_ptr(), message.len());
    }
}

mod ffi {
    extern "C" {
        pub fn on_panic(ptr: *const u8, len: usize);
    }
}
