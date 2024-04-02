use std::collections::BTreeSet;

use super::{symbols::Symbols, syntax::SyntaxElement};

pub fn compile(
    name: &'static str,
    syntax: Vec<SyntaxElement>,
    functions: &BTreeSet<&'static str>,
    symbols: &mut Symbols,
    instructions: &mut Vec<Instruction>,
) {
    let address = instructions.len();

    instructions.extend(syntax.into_iter().map(|expression| {
        match expression {
            SyntaxElement::Value(value) => Instruction::PushValue(value),
            SyntaxElement::Word { name } => {
                // The code here would allow user-defined functions to shadow
                // built-in functions, which seems undesirable. It's better to
                // catch this when defining the function though, and while it
                // would be nice to have a fallback assertion here, that's not
                // practical, given the way built-in function resolution is
                // implemented right now.
                if functions.contains(name) {
                    return Instruction::CallFunction { name };
                }

                // This doesn't check whether the built-in function exists, and
                // given how built-in functions are currently defined, it's not
                // practical to implement.
                Instruction::CallBuiltin { name }
            }
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
