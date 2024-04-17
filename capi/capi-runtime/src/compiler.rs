use std::collections::BTreeSet;

use crate::syntax::Expression;

use super::{code::Code, syntax::ExpressionKind};

pub fn compile(
    name: String,
    syntax: Vec<Expression>,
    functions: &BTreeSet<String>,
    code: &mut Code,
) {
    let address = code.instructions.len();

    for expression in syntax {
        match expression.kind {
            ExpressionKind::Value(value) => {
                code.instructions.push(Instruction::PushValue(value));
            }
            ExpressionKind::Word { name } => {
                // Here we check for special built-in functions that are
                // implemented differently, without making sure anywhere,
                // that its name doesn't conflict with any user-defined
                // functions.
                //
                // I think it's fine for now. This seems like a temporary
                // hack anyway, while the language is not powerful enough to
                // support an actual `if`.
                if name == "return_if_non_zero" {
                    code.instructions.push(Instruction::ReturnIfNonZero);
                    continue;
                }
                if name == "return_if_zero" {
                    code.instructions.push(Instruction::ReturnIfZero);
                    continue;
                }

                // The code here would allow user-defined functions to
                // shadow built-in functions, which seems undesirable. It's
                // better to catch this when defining the function though,
                // and while it would be nice to have a fallback assertion
                // here, that's not practical, given the way built-in
                // function resolution is implemented right now.
                if functions.contains(&name) {
                    code.instructions.push(Instruction::CallFunction { name });
                    continue;
                }

                // This doesn't check whether the built-in function exists,
                // and given how built-in functions are currently defined,
                // it's not practical to implement.
                code.instructions.push(Instruction::CallBuiltin { name });
            }
        }
    }

    code.instructions.push(Instruction::Return);

    code.symbols.define(name, address);
}

#[derive(Clone, Debug)]
pub enum Instruction {
    CallBuiltin { name: String },
    CallFunction { name: String },
    PushValue(usize),
    Return,
    ReturnIfNonZero,
    ReturnIfZero,
}
