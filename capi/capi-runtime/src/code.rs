use super::{compiler::Instruction, symbols::Symbols};

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Code {
    pub symbols: Symbols,
    pub instructions: Vec<Instruction>,
}

impl Code {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, instruction: Instruction) -> usize {
        let address = self.instructions.len();
        self.instructions.push(instruction);
        address
    }
}
