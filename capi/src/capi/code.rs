use super::{compiler::Instruction, symbols::Symbols};

pub struct Code {
    pub symbols: Symbols,
    pub instructions: Vec<Instruction>,
}
