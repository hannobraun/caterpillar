use std::{panic, sync::Mutex};

use crate::{
    cells::Cells, ffi_out::print, input::Input, render_target::RenderTarget,
    state::State, world::World,
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
    let input = match key {
        0 => Input::Up,
        1 => Input::Left,
        2 => Input::Down,
        3 => Input::Right,
        _ => return,
    };

    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    if input == Input::Up && state.world.velocity != [0, 1] {
        state.world.velocity = [0, -1];
    }
    if input == Input::Left && state.world.velocity != [1, 0] {
        state.world.velocity = [-1, 0];
    }
    if input == Input::Down && state.world.velocity != [0, -1] {
        state.world.velocity = [0, 1];
    }
    if input == Input::Right && state.world.velocity != [-1, 0] {
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
