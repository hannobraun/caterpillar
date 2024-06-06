use std::collections::VecDeque;

use super::{instructions::InstructionIndex, Instruction, Location};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub instructions: VecDeque<(InstructionIndex, Instruction)>,
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
        self.instructions
            .front()
            .cloned()
            .map(|(index, instruction)| {
                let location = Location {
                    function: self.name.clone(),
                    index,
                };
                (location, instruction)
            })
    }

    pub fn consume_next_instruction(
        &mut self,
    ) -> Option<(Location, Instruction)> {
        self.instructions.pop_front().map(|(index, instruction)| {
            let location = Location {
                function: self.name.clone(),
                index,
            };
            (location, instruction)
        })
    }
}
