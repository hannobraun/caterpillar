use super::{resolver::Expression, symbols::Symbols};

pub fn compile(
    name: &'static str,
    expressions: Vec<Expression>,
    symbols: &mut Symbols,
    instructions: &mut Vec<Instruction>,
) {
    let address = instructions.len();

    instructions.extend(expressions.into_iter().map(|syntax_element| {
        match syntax_element {
            Expression::Word { name } => {
                // The code here would allow user-defined functions to shadow
                // built-in functions, which seems undesirable. It's better to
                // catch this when defining the function though, and while it
                // would be nice to have a fallback assertion here, that's not
                // practical, given the way built-in function resolution is
                // implemented right now.

                if let Some(address) = symbols.resolve(name) {
                    return Instruction::CallFunction { address };
                }

                Instruction::CallBuiltin { name }
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
