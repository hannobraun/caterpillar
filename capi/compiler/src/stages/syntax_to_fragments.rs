use std::collections::BTreeSet;

use crate::syntax::{Expression, ExpressionKind, Location, Script};

pub fn syntax_to_fragments(script: Script) -> Fragments {
    let mut fragments = Vec::new();

    for function in script.functions.inner {
        fragments.push(Function {
            name: function.name,
            args: function.args,
            expressions: function.expressions,
        });
    }

    Fragments {
        functions: script.functions.names,
        by_function: fragments,
    }
}

#[derive(Debug)]
pub struct Fragments {
    pub functions: BTreeSet<String>,
    pub by_function: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub expressions: Vec<Expression>,
}

#[derive(Debug)]
pub struct Fragment {
    pub kind: ExpressionKind,
    pub location: Location,
}
