use std::collections::VecDeque;

use crate::{
    games::{self, snake::snake},
    program::Program,
    runner::{runner, EventsTx, RunnerHandle},
    updates::{UpdatesRx, UpdatesTx},
};

pub struct RuntimeState {
    pub input: Input,
    pub updates: Updates,
    pub runner: Runner,
}

impl Default for RuntimeState {
    fn default() -> Self {
        let program = games::build(snake);

        let input = Input::default();
        let updates = {
            let (tx, rx) = crate::updates::updates(&program);
            Updates { rx, tx }
        };
        let runner = Runner::new(program, updates.tx.clone());

        Self {
            input,
            updates,
            runner,
        }
    }
}

#[derive(Default)]
pub struct Input {
    pub buffer: VecDeque<u8>,
}

pub struct Updates {
    pub rx: UpdatesRx,
    pub tx: UpdatesTx,
}

pub struct Runner {
    pub events_tx: EventsTx,
    pub handle: Option<RunnerHandle>,
}

impl Runner {
    fn new(program: Program, updates_tx: UpdatesTx) -> Self {
        let (events_tx, handle) = runner(program, updates_tx);

        Self {
            events_tx,
            handle: Some(handle),
        }
    }
}
