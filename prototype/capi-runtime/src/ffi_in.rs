use std::{panic, sync::Mutex};

use crate::{
    cells::Cells, ffi_out::print, render_target::RenderTarget, state::State,
    world::World,
};

static STATE: Mutex<Option<State>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn on_init(width: usize, height: usize) {
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
        .as_mut_ptr();
}

#[no_mangle]
pub extern "C" fn get_render_target_buffer() -> *mut u8 {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.render_target.buffer.as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn get_render_target_buffer_len() -> usize {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.render_target.buffer.len()
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
    state.render_target.draw(&state.world);
}
