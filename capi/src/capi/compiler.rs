use super::{resolver::Expression, symbols::Symbols};

pub fn compile(
    name: &'static str,
    expressions: Vec<Expression>,
    symbols: &mut Symbols,
    instructions: &mut Vec<Instruction>,
) {
    let address = instructions.len();

    instructions.extend(expressions.into_iter().map(|expression| {
        match expression {
            Expression::Builtin { name } => Instruction::CallBuiltin { name },
            Expression::Function { name } => Instruction::CallFunction { name },
            Expression::Value(value) => Instruction::PushValue(value),
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
