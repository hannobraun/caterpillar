use std::{collections::VecDeque, sync::Mutex};

pub static STATE: Mutex<Option<VecDeque<u8>>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn on_key(key_code: u8) {
    let mut state = STATE.lock().unwrap();
    let state = state.get_or_insert_with(Default::default);
    state.push_back(key_code);
}

#[no_mangle]
pub extern "C" fn on_frame() {
    log::debug!("on_frame");
}
