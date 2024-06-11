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
        let runner = {
            let (events_tx, handle) = runner(program, updates_tx);

            Runner {
                events_tx,
                handle: Some(handle),
            }
        };

        let set_program = ui::start(runner.events_tx.clone());
        leptos::spawn_local(handle_updates(updates_rx, set_program));

        Self { input, runner }
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
