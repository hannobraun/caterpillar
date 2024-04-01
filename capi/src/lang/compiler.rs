use super::{symbols::Symbols, syntax::SyntaxElement};

pub fn compile(
    name: &'static str,
    syntax: Vec<SyntaxElement>,
    symbols: &mut Symbols,
    instructions: &mut Vec<Instruction>,
) -> usize {
    let address = instructions.len();

    instructions.extend(syntax.into_iter().map(|syntax_element| {
        match syntax_element {
            SyntaxElement::Word { name } => {
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
            SyntaxElement::Value(value) => Instruction::PushValue(value),
        }
    }));
    instructions.push(Instruction::Return);

    symbols.define(name, address);

    address
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    CallBuiltin { name: &'static str },
    CallFunction { address: usize },
    PushValue(usize),
    Return,
}
