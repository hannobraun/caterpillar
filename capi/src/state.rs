use std::collections::VecDeque;

use crate::{
    games::{self, snake::snake},
    program::Program,
    runner::{runner, EventsTx, RunnerHandle},
    updates::{UpdatesRx, UpdatesTx},
};

pub struct RuntimeState {
    pub input: Input,
    pub updates_rx: UpdatesRx,
    pub updates_tx: UpdatesTx,
    pub runner: Runner,
}

impl Default for RuntimeState {
    fn default() -> Self {
        let program = games::build(snake);

        let input = Input::default();
        let (updates_tx, updates_rx) = crate::updates::updates(&program);
        let runner = Runner::new(program, updates_tx.clone());

        Self {
            input,
            updates_rx,
            updates_tx,
            runner,
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

impl Runner {
    fn new(program: Program, updates_tx: UpdatesTx) -> Self {
        let (events_tx, handle) = runner(program, updates_tx);

        Self {
            events_tx,
            handle: Some(handle),
        }
    }
}
