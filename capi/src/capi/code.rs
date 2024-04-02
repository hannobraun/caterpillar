use super::{compiler::Instruction, symbols::Symbols};

pub struct Code {
    pub symbols: Symbols,
    pub instructions: Vec<Instruction>,
}

impl Code {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            symbols: Symbols::new(),
        }
    }
}
