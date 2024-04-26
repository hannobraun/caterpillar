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

    pub fn next_address(&self) -> InstructionAddress {
        InstructionAddress(self.instructions.len())
    }

    pub fn push(&mut self, instruction: Instruction) -> InstructionAddress {
        let address = self.next_address();
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

impl InstructionAddress {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn to_usize(self) -> usize {
        self.0
    }
}
