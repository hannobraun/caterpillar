mod cells;
mod ffi_out;
mod render_target;
mod state;
mod world;

use std::{panic, sync::Mutex};

use state::State;

use self::{
    cells::Cells, ffi_out::print, render_target::RenderTarget, world::World,
};

static STATE: Mutex<Option<State>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn on_init(width: usize, height: usize) -> *mut u8 {
    panic::set_hook(Box::new(|panic_info| {
        print(&format!("{panic_info}"));
    }));

    let render_target = RenderTarget::new(width, height);
    let cells = Cells::new(&render_target);
    let state = World::new(cells);

    let state = State {
        world: state,
        render_target,
    };

    STATE
        .lock()
        .expect("Expected exclusive access")
        .insert(state)
        .render_target
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

    if key == UP && state.world.velocity != [0, 1] {
        state.world.velocity = [0, -1];
    }
    if key == LEFT && state.world.velocity != [1, 0] {
        state.world.velocity = [-1, 0];
    }
    if key == DOWN && state.world.velocity != [0, -1] {
        state.world.velocity = [0, 1];
    }
    if key == RIGHT && state.world.velocity != [-1, 0] {
        state.world.velocity = [1, 0];
    }
}

#[no_mangle]
pub extern "C" fn on_frame(delta_time_ms: f64) {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.world.update(delta_time_ms);
}

#[no_mangle]
pub extern "C" fn draw() {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    for x in 0..state.world.cells.size[0] {
        for y in 0..state.world.cells.size[1] {
            let base_i = x * state.world.cells.cell_size;
            let base_j = y * state.world.cells.cell_size;

            let color =
                state.world.cells.buffer[x + y * state.world.cells.size[0]];

            draw_cell(
                state.world.cells.cell_size,
                base_i,
                base_j,
                color,
                &mut state.render_target,
            );
        }
    }
}

fn draw_cell(
    cell_size: usize,
    cell_x: usize,
    base_y: usize,
    color: u8,
    target: &mut RenderTarget,
) {
    for x in 0..cell_size {
        for y in 0..cell_size {
            let pixel_x = cell_x + x;
            let abs_j = base_y + y;

            let index = pixel_x + abs_j * target.width;
            target.buffer[index] = color;
        }
    }
}
