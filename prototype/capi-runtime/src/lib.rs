mod cells;
mod draw_target;
mod ffi_out;
mod state;

use std::{panic, sync::Mutex};

use state::State;

use self::{cells::Cells, draw_target::DrawTarget, ffi_out::print};

static DRAW_TARGET: Mutex<Option<DrawTarget>> = Mutex::new(None);
static STATE: Mutex<Option<State>> = Mutex::new(None);
static CELLS: Mutex<Option<Cells>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn on_init(width: usize, height: usize) -> *mut u8 {
    panic::set_hook(Box::new(|panic_info| {
        print(&format!("{panic_info}"));
    }));

    let draw_target = DrawTarget::new(width, height);
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

    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    if key == UP && state.velocity != [0, 1] {
        state.velocity = [0, -1];
    }
}

#[no_mangle]
pub extern "C" fn init_cells(cell_size: usize) -> *mut u8 {
    let mut target = DRAW_TARGET.lock().expect("Expected exclusive access");
    let target = target.as_mut().expect("Expected target to be initialized");

    let cells = Cells::new(cell_size, &target);
    let cells_ptr = CELLS
        .lock()
        .expect("Expected exclusive access")
        .insert(cells)
        .buffer
        .as_mut_ptr();

    let mut cells = CELLS.lock().expect("Expected exclusive access");
    let cells = cells.as_mut().expect("Expected cells to be initialized");

    let state = State::new(&cells);
    *STATE.lock().expect("Expected exclusive access") = Some(state);

    cells_ptr
}

#[no_mangle]
pub extern "C" fn get_velocity_x() -> i32 {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.velocity[0]
}

#[no_mangle]
pub extern "C" fn get_velocity_y() -> i32 {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.velocity[1]
}

#[no_mangle]
pub extern "C" fn set_velocity(vel_x: i32, vel_y: i32) {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.velocity = [vel_x, vel_y];
}

#[no_mangle]
pub extern "C" fn move_snake() {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    let [mut head_x, mut head_y] = state.head_position();

    head_x += state.velocity[0];
    head_y += state.velocity[1];

    state.positions.push_front([head_x, head_y]);

    if state.growth_left > 0 {
        state.growth_left -= 1;
    } else {
        state.positions.pop_back();
    }
}

#[no_mangle]
pub extern "C" fn constrain_positions() {
    let mut cells = CELLS.lock().expect("Expected exclusive access");
    let cells = cells.as_mut().expect("Expected cells to be initialized");

    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    for [x, y] in &mut state.positions {
        if *x < 0 {
            *x = cells.size[0] as i32 - 1;
        }
        if *x >= cells.size[0] as i32 {
            *x = 0;
        }
        if *y < 0 {
            *y = cells.size[1] as i32 - 1;
        }
        if *y >= cells.size[1] as i32 {
            *y = 0;
        }
    }
}

#[no_mangle]
pub extern "C" fn check_collision() -> bool {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    let [head_x, head_y] = state.head_position();

    for [body_x, body_y] in state.body_positions() {
        if head_x == body_x && head_y == body_y {
            return true;
        }
    }

    false
}

#[no_mangle]
pub extern "C" fn eat_food() {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    let mut cells = CELLS.lock().expect("Expected exclusive access");
    let cells = cells.as_mut().expect("Expected cells to be initialized");

    let mut ate_food = false;

    for &[pos_x, pos_y] in &state.positions {
        if pos_x == state.food_pos[0] && pos_y == state.food_pos[1] {
            let extra_growth = state.positions.len() / 2;
            state.growth_left += extra_growth as i32;

            ate_food = true;
        }
    }

    if ate_food {
        state.randomize_food_pos(cells);
    }
}

#[no_mangle]
pub extern "C" fn update_cells() {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    let mut cells = CELLS.lock().expect("Expected exclusive access");
    let cells = cells.as_mut().expect("Expected cells to be initialized");

    for i in 0..cells.buffer.len() {
        cells.buffer[i] = 0;
    }

    for x in 0..cells.size[0] {
        for y in 0..cells.size[1] {
            let index = x + y * cells.size[0];

            if x as i32 == state.food_pos[0] && y as i32 == state.food_pos[1] {
                cells.buffer[index] = 127;
            }

            for &[pos_x, pos_y] in &state.positions {
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

    for x in 0..cells.size[0] {
        for y in 0..cells.size[1] {
            let base_i = x * cells.cell_size;
            let base_j = y * cells.cell_size;

            let color = cells.buffer[x + y * cells.size[0]];

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
