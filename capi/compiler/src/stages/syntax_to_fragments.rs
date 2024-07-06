use std::collections::BTreeMap;

use crate::syntax::{Expression, Script};

pub fn syntax_to_fragments(script: Script) -> Fragments {
    let mut fragments = BTreeMap::new();

    for function in script.functions.inner {
        let mut function_fragments = Vec::new();

        for expression in function.expressions {
            function_fragments.push(expression);
        }

        fragments.insert(function.name, function_fragments);
    }

    Fragments {
        by_function: fragments,
    }
}

#[derive(Debug)]
pub struct Fragments {
    pub by_function: BTreeMap<String, Vec<Expression>>,
}
