use std::sync::Mutex;

use crate::state::RuntimeState;

pub static STATE: Mutex<Option<RuntimeState>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn on_key(key_code: u8) {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    state.input.buffer.push_back(key_code);
}

#[no_mangle]
pub extern "C" fn on_frame() {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);

    state.update();
}
