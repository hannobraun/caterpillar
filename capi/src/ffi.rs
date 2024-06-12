use std::sync::Mutex;

use crate::state::RuntimeState;

pub static STATE: StaticRuntimeState = StaticRuntimeState {
    inner: Mutex::new(None),
};

#[no_mangle]
pub extern "C" fn on_key(key_code: u8) {
    let mut state = STATE.inner.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    state.input.buffer.push_back(key_code);
}

#[no_mangle]
pub extern "C" fn on_frame() {
    let mut state = STATE.inner.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    let Some(display) = state.display.as_mut() else {
        // Display not initialized yet.
        return;
    };

    while let Ok(effect) = state.effects.try_recv() {
        display.handle_effects(&mut state.input, effect, &mut state.tiles);
    }
    display.render(&state.tiles);
}

pub struct StaticRuntimeState {
    pub inner: Mutex<Option<RuntimeState>>,
}
