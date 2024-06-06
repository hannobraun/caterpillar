use std::collections::VecDeque;

use super::{Instruction, Location};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub instructions: VecDeque<(Location, Instruction)>,
}

impl Function {
    pub fn new(name: String, arguments: Vec<String>) -> Self {
        Self {
            name,
            arguments,
            instructions: VecDeque::new(),
        }
    }

    pub fn next_instruction(&self) -> Option<(Location, Instruction)> {
        self.instructions.front().cloned()
    }

    pub fn consume_instruction(&mut self) -> Option<(Location, Instruction)> {
        self.instructions.pop_front()
    }
}
