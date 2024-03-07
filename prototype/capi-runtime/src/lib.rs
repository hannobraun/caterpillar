use std::mem;

#[no_mangle]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[no_mangle]
pub extern "C" fn allocate_draw_buffer(
    canvas_width: usize,
    canvas_height: usize,
) -> *mut u8 {
    const NUM_COLOR_CHANNELS: usize = 4;
    let len = canvas_width * canvas_height * NUM_COLOR_CHANNELS;

    let mut buffer = Vec::with_capacity(len);
    let ptr = buffer.as_mut_ptr();
    mem::forget(buffer);

    ptr
}
