use std::collections::BTreeMap;

use crate::{
    fragments::{
        Changes, Fragments, FunctionInUpdate, FunctionUpdate, NamedFunctions,
    },
    hash::Hash,
};

pub fn detect_changes(old: Option<NamedFunctions>, new: &Fragments) -> Changes {
    let mut old_functions = old.unwrap_or_default();
    let mut new_functions = new.named_functions.inner.clone();

    let mut added = BTreeMap::new();
    let mut updated = Vec::new();

    while let Some((new_index, new_function)) = new_functions.pop_first() {
        if old_functions
            .find_by_hash(&Hash::new(&new_function))
            .is_some()
        {
            // Function has not changed. We can forget about it.
            continue;
        }

        let name = new_function
            .name
            .as_deref()
            .expect("Named function should have a name.");
        if let Some(same_name) = old_functions.find_by_name(name) {
            let old_index = same_name.metadata;

            // Found a function with the same name. But it can't have the same
            // hash, or we wouldn't have made it here. Assuming the new function
            // is an updated version of the old.
            let old_function = old_functions.inner.remove(&old_index).expect(
                "Just found index in map; expecting it to still be there.",
            );
            updated.push(FunctionUpdate {
                old: FunctionInUpdate {
                    index: old_index,
                    function: old_function,
                },
                new: FunctionInUpdate {
                    index: new_index,
                    function: new_function,
                },
            });

            continue;
        }

        // If we make it here, there was neither an identical function before,
        // nor one with the same name. This must mean this function is new.
        added.insert(new_index, new_function);
    }

    Changes { added, updated }
}
