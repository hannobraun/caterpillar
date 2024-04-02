use std::collections::BTreeSet;

use super::{compiler::Instruction, syntax::SyntaxElement};

pub fn resolve(
    syntax: Vec<SyntaxElement>,
    functions: &BTreeSet<&'static str>,
) -> Vec<Instruction> {
    syntax
        .into_iter()
        .map(|syntax_element| match syntax_element {
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
        })
        .collect()
}
