use std::collections::VecDeque;

use crate::{
    games::{self, snake::snake},
    program::Program,
};

#[derive(Default)]
pub struct RuntimeState {
    pub input: Input,
    pub game: Game,
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
