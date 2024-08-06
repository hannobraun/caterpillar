use std::collections::BTreeMap;

use crate::repr::syntax::Function;

pub fn group_functions(functions: &mut Vec<Function>) {
    let mut groups = BTreeMap::new();

    for function in functions {
        let next_group_index = groups.entry(function.name.clone()).or_default();
        function.group_index = Some(*next_group_index);
        *next_group_index += 1;
    }
}
