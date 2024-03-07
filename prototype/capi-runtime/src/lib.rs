mod cells;
mod draw_target;

use std::{panic, sync::Mutex};

use self::{cells::Cells, draw_target::DrawTarget};

static DRAW_TARGET: Mutex<Option<DrawTarget>> = Mutex::new(None);
static CELLS: Mutex<Option<Cells>> = Mutex::new(None);

extern "C" {
    fn print_ffi(ptr: *const u8, len: usize);
}

#[no_mangle]
pub extern "C" fn init() {
    panic::set_hook(Box::new(|panic_info| {
        let msg = format!("{panic_info}");
        unsafe {
            print_ffi(msg.as_ptr(), msg.len());
        }
    }));
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
pub extern "C" fn init_cells(cell_size: usize) -> *mut u8 {
    let mut target = DRAW_TARGET.lock().expect("Expected exclusive access");
    let target = target.as_mut().expect("Expected target to be initialized");

    let cells = Cells::new(cell_size, &target);
    CELLS
        .lock()
        .expect("Expected exclusive access")
        .insert(cells)
        .buffer
        .as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn draw() {
    let mut cells = CELLS.lock().expect("Expected exclusive access");
    let cells = cells.as_mut().expect("Expected cells to be initialized");

    for x in 0..cells.width {
        for y in 0..cells.height {
            let base_i = x * cells.cell_size;
            let base_j = y * cells.cell_size;

            let color = cells.buffer[x + y * cells.width];

            draw_cell(cells.cell_size, base_i, base_j, color);
        }
    }
}

fn draw_cell(cell_size: usize, base_i: usize, base_j: usize, color: u8) {
    let mut target = DRAW_TARGET.lock().expect("Expected exclusive access");
    let target = target.as_mut().expect("Expected target to be initialized");

    for i in 0..cell_size {
        for j in 0..cell_size {
            let abs_i = base_i + i;
            let abs_j = base_j + j;

            let index = abs_i + abs_j * target.width;
            target.buffer[index] = color;
        }
    }
}
