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
        let input = Input::default();
        let game = Game {
            program: games::build(snake),
        };
        let updates = Updates::new(&game.program);
        let runner = Runner::new(game.program.clone(), updates.tx.clone());

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

pub struct Game {
    pub program: Program,
}

pub struct Updates {
    pub rx: UpdatesRx,
    pub tx: UpdatesTx,
}

impl Updates {
    pub fn new(program: &Program) -> Self {
        let (tx, rx) = crate::updates::updates(program);
        Self { rx, tx }
    }
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
