macro_rules! println {
    ($($arg:tt)*) => {{
        use alloc::string::ToString;
        let s = core::format_args!($($arg)*).to_string();
        $crate::ffi_out::console_log(&s);
    }};
}
