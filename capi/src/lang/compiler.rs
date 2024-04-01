use super::{functions::Symbols, syntax::SyntaxElement};

pub fn compile(
    syntax: Vec<SyntaxElement>,
    functions: &Symbols,
    instructions: &mut Vec<Instruction>,
) {
    instructions.extend(syntax.into_iter().map(|syntax_element| {
        match syntax_element {
            SyntaxElement::Word { name } => {
                // The code here would allow user-defined functions to shadow
                // built-in functions, which seems undesirable. It's better to
                // catch this when defining the function though, and while it
                // would be nice to have a fallback assertion here, that's not
                // practical, given the way built-in function resolution is
                // implemented right now.

                if let Some(address) = functions.resolve(name) {
                    return Instruction::CallFunction { address };
                }

                Instruction::CallBuiltin { name }
            }
            SyntaxElement::Value(value) => Instruction::PushValue(value),
        }
    }));
    instructions.push(Instruction::Return);
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    CallBuiltin { name: &'static str },
    CallFunction { address: usize },
    PushValue(usize),
    Return,
}
