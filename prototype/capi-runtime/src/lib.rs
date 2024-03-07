use std::{iter, panic, sync::Mutex};

static DRAW_BUFFER: Mutex<Option<DrawTarget>> = Mutex::new(None);

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

pub struct DrawTarget {
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

#[no_mangle]
pub extern "C" fn init_draw_target(
    canvas_width: usize,
    canvas_height: usize,
) -> *mut u8 {
    const NUM_COLOR_CHANNELS: usize = 4;
    let len = canvas_width * canvas_height * NUM_COLOR_CHANNELS;

    let buffer = DrawTarget {
        buffer: iter::repeat(0).take(len).collect(),
        width: canvas_width,
        height: canvas_height,
    };
    DRAW_BUFFER
        .lock()
        .expect(
            "Expected exclusive access in single-threaded WebAssembly context",
        )
        .insert(buffer)
        .buffer
        .as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn extern_draw_cell(
    cell_size: usize,
    base_i: usize,
    base_j: usize,
    color: u8,
) {
    let mut guard = DRAW_BUFFER.lock().unwrap();
    let buffer = guard.as_mut().unwrap();

    draw_cell(cell_size, base_i, base_j, color, buffer)
}

fn draw_cell(
    cell_size: usize,
    base_i: usize,
    base_j: usize,
    color: u8,
    buffer: &mut DrawTarget,
) {
    for i in 0..cell_size {
        for j in 0..cell_size {
            let abs_i = base_i + i;
            let abs_j = base_j + j;

            let index = abs_i + abs_j * buffer.width;
            buffer.buffer[index] = color;
        }
    }
}
