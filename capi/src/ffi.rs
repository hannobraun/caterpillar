use std::sync::Mutex;

use crate::{effects::DisplayEffect, state::RuntimeState};

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

    while let Ok(effect) = state.effects_rx.try_recv() {
        match effect {
            DisplayEffect::SetTile { x, y, value } => {
                display.set_tile(x.into(), y.into(), value, &mut state.tiles);
            }
            DisplayEffect::SubmitTiles { reply } => {
                reply.send(()).unwrap();
            }
            DisplayEffect::ReadInput { reply } => {
                let input = state
                    .input
                    .buffer
                    .pop_front()
                    .unwrap_or(0)
                    .try_into()
                    .unwrap();
                reply.send(input).unwrap();
            }
        };
    }

    display.render(&state.tiles);
}

pub struct StaticRuntimeState {
    pub inner: Mutex<Option<RuntimeState>>,
}
