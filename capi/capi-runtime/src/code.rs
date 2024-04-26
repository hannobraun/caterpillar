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

    pub fn push(&mut self, instruction: Instruction) -> InstructionAddress {
        let address = InstructionAddress(self.instructions.len());
        self.instructions.push(instruction);
        address
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct InstructionAddress(pub usize);
