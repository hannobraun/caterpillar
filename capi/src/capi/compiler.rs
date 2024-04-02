use super::symbols::Symbols;

pub fn compile(
    name: &'static str,
    expressions: Vec<Instruction>,
    symbols: &mut Symbols,
    instructions: &mut Vec<Instruction>,
) {
    let address = instructions.len();

    instructions.extend(expressions.into_iter().map(|expression| {
        match expression {
            Instruction::CallBuiltin { name } => {
                Instruction::CallBuiltin { name }
            }
            Instruction::CallFunction { name } => {
                Instruction::CallFunction { name }
            }
            Instruction::PushValue(value) => Instruction::PushValue(value),
            Instruction::Return => Instruction::Return,
        }
    }));
    instructions.push(Instruction::Return);

    symbols.define(name, address);
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    CallBuiltin { name: &'static str },
    CallFunction { name: &'static str },
    PushValue(usize),
    Return,
}
