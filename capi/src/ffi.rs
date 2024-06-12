use std::sync::Mutex;

use rand::random;
use tokio::sync::mpsc::error::TryRecvError;

use crate::{
    program::{ProgramEffect, ProgramEffectKind},
    runtime::{BuiltinEffect, EvaluatorEffectKind, Value},
    state::RuntimeState,
};

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

    loop {
        match state.events_rx.try_recv() {
            Ok(event) => {
                state.program.process_event(event);
            }
            Err(TryRecvError::Empty) => {
                break;
            }
            Err(TryRecvError::Disconnected) => {
                // The other end has hung up, which happens during
                // shutdown. Shut down this task, too.
                return;
            }
        }
    }

    while state.program.can_step() {
        state.program.step();

        if let Some(ProgramEffect {
            kind:
                ProgramEffectKind::Evaluator(EvaluatorEffectKind::Builtin(effect)),
            ..
        }) = state.program.effects.front()
        {
            match effect {
                BuiltinEffect::Error(_) => {
                    // Nothing needs to be done. With an unhandled
                    // effect, the program won't continue running, and
                    // the debugger will see the error and display it.
                }
                BuiltinEffect::Load { address } => {
                    let address: usize = (*address).into();
                    let value = state.program.memory.inner[address];
                    state.program.push([value]);

                    state.program.effects.pop_front();
                }
                BuiltinEffect::Store { address, value } => {
                    let address: usize = (*address).into();
                    state.program.memory.inner[address] = *value;

                    state.program.effects.pop_front();
                }
                BuiltinEffect::SetTile { x, y, value } => {
                    let x = *x;
                    let y = *y;
                    let value = *value;

                    state.program.effects.pop_front();

                    display.set_tile(
                        x.into(),
                        y.into(),
                        value,
                        &mut state.tiles,
                    );
                }
                BuiltinEffect::SubmitFrame => {
                    // This effect means that the game is done rendering. Let's
                    // break out of this loop now, so we can do our part in that
                    // and return control to the host.
                    state.program.effects.pop_front();
                    break;
                }
                BuiltinEffect::ReadInput => {
                    let input = state
                        .input
                        .buffer
                        .pop_front()
                        .unwrap_or(0)
                        .try_into()
                        .unwrap();

                    state.program.push([Value(input)]);
                    state.program.effects.pop_front();
                }
                BuiltinEffect::ReadRandom => {
                    state.program.push([Value(random())]);
                    state.program.effects.pop_front();
                }
            }
        }
    }

    state.updates_tx.send_if_relevant_change(&state.program);

    display.render(&state.tiles);
}

pub struct StaticRuntimeState {
    pub inner: Mutex<Option<RuntimeState>>,
}
