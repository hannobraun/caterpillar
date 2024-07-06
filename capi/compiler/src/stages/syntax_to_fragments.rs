use std::collections::BTreeSet;

use crate::syntax::{ExpressionKind, Location, Script};

pub fn syntax_to_fragments(script: Script) -> Fragments {
    let mut by_function = Vec::new();

    for function in script.functions.inner {
        let mut fragments = Vec::new();

        for expression in function.expressions {
            fragments.push(Fragment {
                kind: expression.kind,
                location: expression.location,
            });
        }

        by_function.push(Function {
            name: function.name,
            args: function.args,
            fragments,
        });
    }

    Fragments {
        functions: script.functions.names,
        by_function,
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
    pub fragments: Vec<Fragment>,
}

#[derive(Debug)]
pub struct Fragment {
    pub kind: ExpressionKind,
    pub location: Location,
}
