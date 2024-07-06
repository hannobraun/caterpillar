use std::collections::BTreeSet;

use crate::syntax::{Function, Script};

pub fn syntax_to_fragments(script: Script) -> Fragments {
    let mut fragments = Vec::new();

    for function in script.functions.inner {
        fragments.push(function);
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
