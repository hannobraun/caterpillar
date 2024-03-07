use std::{iter, panic, sync::Mutex};

static DRAW_TARGET: Mutex<Option<DrawTarget>> = Mutex::new(None);

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

impl DrawTarget {
    pub fn new(width: usize, height: usize) -> Self {
        const NUM_COLOR_CHANNELS: usize = 4;
        let len = width * height * NUM_COLOR_CHANNELS;

        let buffer = iter::repeat(0).take(len).collect();

        Self {
            buffer,
            width,
            height,
        }
    }
}

#[no_mangle]
pub extern "C" fn init_draw_target(width: usize, height: usize) -> *mut u8 {
    let buffer = DrawTarget::new(width, height);
    DRAW_TARGET
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
    let mut target = DRAW_TARGET.lock().expect("Expected exclusive access");
    let target = target.as_mut().expect("Expected target to be initialized");

    draw_cell(cell_size, base_i, base_j, color, target)
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
