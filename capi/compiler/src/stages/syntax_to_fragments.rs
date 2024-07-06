use std::collections::BTreeSet;

use capi_process::Value;

use crate::syntax::{ExpressionKind, Location, Script};

pub fn syntax_to_fragments(script: Script) -> Fragments {
    let mut by_function = Vec::new();

    for function in script.functions.inner {
        let mut fragments = Vec::new();

        for expression in function.expressions {
            let payload = match expression.kind {
                ExpressionKind::Binding { names } => {
                    FragmentPayload::Binding { names }
                }
                ExpressionKind::Comment { text } => {
                    FragmentPayload::Comment { text }
                }
                ExpressionKind::Value(value) => FragmentPayload::Value(value),
                ExpressionKind::Word { name } => FragmentPayload::Word { name },
            };

            fragments.push(Fragment {
                payload,
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
    pub payload: FragmentPayload,
    pub location: Location,
}

#[derive(Debug)]
pub enum FragmentPayload {
    Binding { names: Vec<String> },
    Comment { text: String },
    Value(Value),
    Word { name: String },
}
