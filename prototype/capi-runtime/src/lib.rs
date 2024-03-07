mod cells;
mod draw_target;
mod ffi_out;

use std::{collections::VecDeque, iter, panic, sync::Mutex};

use self::{cells::Cells, draw_target::DrawTarget, ffi_out::print};

static DRAW_TARGET: Mutex<Option<DrawTarget>> = Mutex::new(None);
static POSITIONS: Mutex<Option<VecDeque<[i32; 2]>>> = Mutex::new(None);
static CELLS: Mutex<Option<Cells>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn init() {
    panic::set_hook(Box::new(|panic_info| {
        print(&format!("{panic_info}"));
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
pub extern "C" fn positions_init(x: i32, y: i32) {
    let positions = iter::once([x, y]).collect();
    *POSITIONS.lock().expect("Expected exclusive access") = Some(positions);
}

#[no_mangle]
pub extern "C" fn positions_len() -> usize {
    let mut positions = POSITIONS.lock().expect("Expected exclusive access");
    let positions = positions
        .as_mut()
        .expect("Expected positions to be initialized");

    positions.len()
}

#[no_mangle]
pub extern "C" fn positions_get_x(i: usize) -> i32 {
    let mut positions = POSITIONS.lock().expect("Expected exclusive access");
    let positions = positions
        .as_mut()
        .expect("Expected positions to be initialized");

    positions[i][0]
}

#[no_mangle]
pub extern "C" fn positions_get_y(i: usize) -> i32 {
    let mut positions = POSITIONS.lock().expect("Expected exclusive access");
    let positions = positions
        .as_mut()
        .expect("Expected positions to be initialized");

    positions[i][1]
}

#[no_mangle]
pub extern "C" fn positions_set_x(i: usize, x: i32) {
    let mut positions = POSITIONS.lock().expect("Expected exclusive access");
    let positions = positions
        .as_mut()
        .expect("Expected positions to be initialized");

    positions[i][0] = x;
}

#[no_mangle]
pub extern "C" fn positions_set_y(i: usize, y: i32) {
    let mut positions = POSITIONS.lock().expect("Expected exclusive access");
    let positions = positions
        .as_mut()
        .expect("Expected positions to be initialized");

    positions[i][1] = y;
}

#[no_mangle]
pub extern "C" fn positions_push_front(x: i32, y: i32) {
    let mut positions = POSITIONS.lock().expect("Expected exclusive access");
    let positions = positions
        .as_mut()
        .expect("Expected positions to be initialized");

    positions.push_front([x, y]);
}

#[no_mangle]
pub extern "C" fn positions_pop_back() {
    let mut positions = POSITIONS.lock().expect("Expected exclusive access");
    let positions = positions
        .as_mut()
        .expect("Expected positions to be initialized");

    positions.pop_back();
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
pub extern "C" fn update_cells(food_x: i32, food_y: i32) {
    let mut positions = POSITIONS.lock().expect("Expected exclusive access");
    let positions = positions
        .as_mut()
        .expect("Expected positions to be initialized");

    let mut cells = CELLS.lock().expect("Expected exclusive access");
    let cells = cells.as_mut().expect("Expected cells to be initialized");

    for x in 0..cells.width {
        for y in 0..cells.height {
            let index = x + y * cells.width;

            if x as i32 == food_x && y as i32 == food_y {
                cells.buffer[index] = 127;
            }

            for &[pos_x, pos_y] in &*positions {
                if x as i32 == pos_x && y as i32 == pos_y {
                    cells.buffer[index] = 255;
                }
            }
        }
    }
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
