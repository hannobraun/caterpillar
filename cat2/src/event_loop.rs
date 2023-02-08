use std::io;

use crate::{cells::Generation, cp::Functions, ui};

pub struct State {
    pub functions: Functions,
    pub generations: Vec<Generation>,
    pub buffer: ui::Buffer,
    pub stdout: io::Stdout,
}
