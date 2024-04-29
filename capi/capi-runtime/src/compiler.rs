use std::collections::BTreeSet;

use crate::{source_map::SourceMap, syntax::Expression, Value};

use super::{code::Code, syntax::ExpressionKind};

pub fn compile(
    name: String,
    syntax: Vec<Expression>,
    functions: &BTreeSet<String>,
    code: &mut Code,
    source_map: &mut SourceMap,
) {
    let address = code.next_address();

    for expression in syntax {
        let instruction = match expression.kind {
            ExpressionKind::Value(value) => Instruction::PushValue(value),
            ExpressionKind::Word { name } => {
                word_to_instruction(name, functions)
            }
        };

        let address = code.push(instruction);
        source_map.define_mapping(address, expression.location)
    }

    code.push(Instruction::Return);

    code.symbols.define(name, address);
}

fn word_to_instruction(
    word: String,
    functions: &BTreeSet<String>,
) -> Instruction {
    // Here we check for special built-in functions that are implemented
    // differently, without making sure anywhere, that its name doesn't conflict
    // with any user-defined functions.
    //
    // I think it's fine for now. This seems like a temporary hack anyway, while
    // the language is not powerful enough to support an actual `if`.
    if word == "return_if_non_zero" {
        return Instruction::ReturnIfNonZero;
    }
    if word == "return_if_zero" {
        return Instruction::ReturnIfZero;
    }

    // The code here would allow user-defined functions to shadow built-in
    // functions, which seems undesirable. It's better to catch this when
    // defining the function though, and while it would be nice to have a
    // fallback assertion here, that's not practical, given the way built-in
    // function resolution is implemented right now.
    if functions.contains(&word) {
        return Instruction::CallFunction { name: word };
    }

    // This doesn't check whether the built-in function exists, and given how
    // built-in functions are currently defined, it's not practical to
    // implement.
    Instruction::CallBuiltin { name: word }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Instruction {
    CallBuiltin { name: String },
    CallFunction { name: String },
    PushValue(Value),
    Return,
    ReturnIfNonZero,
    ReturnIfZero,
}
