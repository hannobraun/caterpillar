use std::collections::BTreeSet;

use super::{code::Code, syntax::SyntaxElement};

pub fn compile(
    name: String,
    syntax: Vec<SyntaxElement>,
    functions: &BTreeSet<String>,
    code: &mut Code,
) {
    let address = code.instructions.len();

    code.instructions
        .extend(syntax.into_iter().map(|expression| {
            match expression {
                SyntaxElement::Value(value) => Instruction::PushValue(value),
                SyntaxElement::Word { name } => {
                    // Here we check for the one special built-in function that
                    // is implemented differently, without making sure anywhere,
                    // that its name doesn't conflict with any user-defined
                    // functions.
                    //
                    // I think it's fine for now. This seems like a temporary
                    // hack anyway, while the language is not powerful enough to
                    // support an actual `if`.
                    if name == "return_if_zero" {
                        return Instruction::ReturnIfZero;
                    }

                    // The code here would allow user-defined functions to
                    // shadow built-in functions, which seems undesirable. It's
                    // better to catch this when defining the function though,
                    // and while it would be nice to have a fallback assertion
                    // here, that's not practical, given the way built-in
                    // function resolution is implemented right now.
                    if functions.contains(&name) {
                        return Instruction::CallFunction { name };
                    }

                    // This doesn't check whether the built-in function exists,
                    // and given how built-in functions are currently defined,
                    // it's not practical to implement.
                    Instruction::CallBuiltin { name }
                }
            }
        }));
    code.instructions.push(Instruction::Return);

    code.symbols.define(name, address);
}

#[derive(Clone, Debug)]
pub enum Instruction {
    CallBuiltin { name: String },
    CallFunction { name: String },
    PushValue(usize),
    Return,
    ReturnIfZero,
}
