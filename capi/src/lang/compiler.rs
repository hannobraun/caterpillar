use super::functions::Functions;

pub fn compile(
    syntax: Vec<Instruction>,
    _: &Functions,
    instructions: &mut Vec<Instruction>,
) {
    instructions.extend(syntax);
    instructions.push(Instruction::Return);
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    CallBuiltin { name: &'static str },
    CallFunction { address: usize },
    PushValue(usize),
    Return,
}
