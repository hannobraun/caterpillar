use std::{panic, sync::Mutex};

use crate::{ffi_out::print, input::InputEvent, state::State};

static STATE: Mutex<Option<State>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn on_init(width: usize, height: usize) {
    panic::set_hook(Box::new(|panic_info| {
        print(&format!("{panic_info}"));
    }));

    let mut state = State::new(width, height);
    state.evaluator.load_program(&[
        b'c', b'p', 0, b'S', b'c', b'p', 1, b'S', b'p', 2, b'S', b'p', 255,
        b'p', 3, b'S', b't',
    ]);

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
        0 => InputEvent::Up,
        1 => InputEvent::Left,
        2 => InputEvent::Down,
        3 => InputEvent::Right,
        _ => return,
    };

    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.world.input.events.push_back(input);
}

#[no_mangle]
pub extern "C" fn on_frame(delta_time_ms: f64) {
    let mut state = STATE.lock().expect("Expected exclusive access");
    let state = state.as_mut().expect("Expected state to be initialized");

    state.world.update(delta_time_ms);
    state.render_target.draw(&state.world, &mut state.evaluator);
}
