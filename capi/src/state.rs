use std::collections::VecDeque;

use crate::{
    games::{self, snake::snake},
    program::Program,
    updates::{UpdatesRx, UpdatesTx},
};

pub struct RuntimeState {
    pub input: Input,
    pub game: Game,
    pub updates: Updates,
}

impl Default for RuntimeState {
    fn default() -> Self {
        let input = Input::default();
        let game = Game::default();
        let updates = Updates::new(&game.program);

        Self {
            input,
            game,
            updates,
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

impl Default for Game {
    fn default() -> Self {
        let program = games::build(snake);
        Self { program }
    }
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
