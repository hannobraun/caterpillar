mod cells;
mod draw_target;
mod ffi_out;
mod state;

use std::{panic, sync::Mutex};

use state::State;

use self::{cells::Cells, draw_target::DrawTarget, ffi_out::print};

static DRAW_TARGET: Mutex<Option<DrawTarget>> = Mutex::new(None);
static STATE: Mutex<Option<State>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn on_init(width: usize, height: usize) -> *mut u8 {
    panic::set_hook(Box::new(|panic_info| {
        print(&format!("{panic_info}"));
    }));

    let draw_target = DrawTarget::new(width, height);

    let cells = Cells::new(&draw_target);
    let state = State::new(cells);
    *STATE.lock().expect("Expected exclusive access") = Some(state);

    DRAW_TARGET
        .lock()
        .expect("Expected exclusive access")
        .insert(draw_target)
        .buffer
        .as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn on_input(key: i32) {
    const UP: i32 = 0;
    const LEFT: i32 = 1;
    const DOWN: i32 = 2;
    const RIGHT: i32 = 3;

    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    if key == UP && state.velocity != [0, 1] {
        state.velocity = [0, -1];
    }
    if key == LEFT && state.velocity != [1, 0] {
        state.velocity = [-1, 0];
    }
    if key == DOWN && state.velocity != [0, -1] {
        state.velocity = [0, 1];
    }
    if key == RIGHT && state.velocity != [-1, 0] {
        state.velocity = [1, 0];
    }
}

#[no_mangle]
pub extern "C" fn on_frame(delta_time_ms: f64) {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.update(delta_time_ms);
}

#[no_mangle]
pub extern "C" fn draw() {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    for x in 0..state.cells.size[0] {
        for y in 0..state.cells.size[1] {
            let base_i = x * state.cells.cell_size;
            let base_j = y * state.cells.cell_size;

            let color = state.cells.buffer[x + y * state.cells.size[0]];

            draw_cell(state.cells.cell_size, base_i, base_j, color);
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
