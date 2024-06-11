use std::sync::Mutex;

use crate::state::RuntimeState;

pub static STATE: StaticRuntimeState = StaticRuntimeState {
    inner: Mutex::new(None),
};

#[no_mangle]
pub extern "C" fn on_key(key_code: u8) {
    let mut state = STATE.inner.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);
    state.input.push_back(key_code);
}

#[no_mangle]
pub extern "C" fn on_frame() {
    log::debug!("on_frame");
}

pub struct StaticRuntimeState {
    pub inner: Mutex<Option<RuntimeState>>,
}
