use super::{compiler::Instruction, symbols::Symbols};

#[derive(Debug, Default)]
pub struct Code {
    pub symbols: Symbols,
    pub instructions: Vec<Instruction>,
}

impl Code {
    pub fn new() -> Self {
        Self::default()
    }
}
