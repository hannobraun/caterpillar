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
            Expression::Function { name } => {
                let address = symbols.resolve(name).unwrap();
                Instruction::CallFunction { address }
            }
            Expression::Value(value) => Instruction::PushValue(value),
        }
    }));
    instructions.push(Instruction::Return);

    symbols.define(name, address);
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    CallBuiltin { name: &'static str },
    CallFunction { address: usize },
    PushValue(usize),
    Return,
}
