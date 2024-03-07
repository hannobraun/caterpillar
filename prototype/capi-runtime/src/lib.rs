use std::{iter, panic, sync::Mutex};

static DRAW_BUFFER: Mutex<Option<Vec<u8>>> = Mutex::new(None);

extern "C" {
    fn print(ptr: *const u8, len: usize);
}

#[no_mangle]
pub extern "C" fn init() {
    panic::set_hook(Box::new(|panic_info| {
        let msg = format!("{panic_info}");
        unsafe {
            print(msg.as_ptr(), msg.len());
        }
    }));
}

#[no_mangle]
pub extern "C" fn init_draw_buffer(
    canvas_width: usize,
    canvas_height: usize,
) -> *mut u8 {
    const NUM_COLOR_CHANNELS: usize = 4;
    let len = canvas_width * canvas_height * NUM_COLOR_CHANNELS;

    let buffer = iter::repeat(0).take(len).collect();
    DRAW_BUFFER
        .lock()
        .expect(
            "Expected exclusive access in single-threaded WebAssembly context",
        )
        .insert(buffer)
        .as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn draw_cell(
    cell_size: usize,
    base_i: usize,
    base_j: usize,
    color: u8,
    width: usize,
) {
    let mut guard = DRAW_BUFFER.lock().unwrap();
    let buffer = guard.as_mut().unwrap();

    for i in 0..cell_size {
        for j in 0..cell_size {
            let abs_i = base_i + i;
            let abs_j = base_j + j;

            let index = abs_i + abs_j * width;
            buffer[index] = color;
        }
    }
}
