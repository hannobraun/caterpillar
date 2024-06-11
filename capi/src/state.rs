use std::collections::VecDeque;

use crate::{
    games::{self, snake::snake},
    handle_updates,
    runner::{runner, EventsTx, RunnerHandle},
    ui,
    updates::updates,
};

pub struct RuntimeState {
    pub input: Input,
    pub runner: Runner,
}

impl Default for RuntimeState {
    fn default() -> Self {
        let program = games::build(snake);

        let input = Input::default();
        let (updates_tx, updates_rx) = updates(&program);
        let (events_tx, handle) = runner(program, updates_tx);

        let set_program = ui::start(events_tx.clone());
        leptos::spawn_local(handle_updates(updates_rx, set_program));

        Self {
            input,
            runner: Runner {
                events_tx,
                handle: Some(handle),
            },
        }
    }
}

#[derive(Default)]
pub struct Input {
    pub buffer: VecDeque<u8>,
}

pub struct Runner {
    pub events_tx: EventsTx,
    pub handle: Option<RunnerHandle>,
}
