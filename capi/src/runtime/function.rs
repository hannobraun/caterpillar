use super::{Instruction, Instructions, Location};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub instructions: Instructions,
}

impl Function {
    pub fn new(name: String, arguments: Vec<String>) -> Self {
        Self {
            name,
            arguments,
            instructions: Instructions::default(),
        }
    }

    pub fn next_instruction(&self) -> Option<(Location, Instruction)> {
        self.instructions.next().map(|(index, instruction)| {
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
        self.instructions
            .consume_next()
            .map(|(index, instruction)| {
                let location = Location {
                    function: self.name.clone(),
                    index,
                };
                (location, instruction)
            })
    }
}
