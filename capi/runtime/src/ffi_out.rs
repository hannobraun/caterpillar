pub unsafe fn on_panic(ptr: *const u8, len: usize) {
    ffi::on_panic(ptr, len);
}

mod ffi {
    extern "C" {
        pub fn on_panic(ptr: *const u8, len: usize);
    }
}
