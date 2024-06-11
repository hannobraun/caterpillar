use std::collections::VecDeque;

use crate::{
    games::{self, snake::snake},
    program::Program,
};

pub struct RuntimeState {
    pub input: Input,
    pub game: Game,
}

impl Default for RuntimeState {
    fn default() -> Self {
        let input = Input::default();
        let game = Game::default();

        Self { input, game }
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
