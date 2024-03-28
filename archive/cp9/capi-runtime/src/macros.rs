macro_rules! println {
    ($($arg:tt)*) => {{
        let s = core::format_args!($($arg)*).to_string();
        $crate::ffi_out::console_log(&s);
    }};
}
