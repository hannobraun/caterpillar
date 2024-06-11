use std::{collections::VecDeque, sync::Mutex};

pub static STATE: StaticRuntimeState = StaticRuntimeState {
    input: Mutex::new(None),
};

#[no_mangle]
pub extern "C" fn on_key(key_code: u8) {
    let mut state = STATE.input.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);
    state.push_back(key_code);
}

#[no_mangle]
pub extern "C" fn on_frame() {
    log::debug!("on_frame");
}

pub struct StaticRuntimeState {
    pub input: Mutex<Option<VecDeque<u8>>>,
}
