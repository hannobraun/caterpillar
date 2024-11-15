use std::collections::BTreeMap;

use crate::code::{
    Changes, FunctionInUpdate, FunctionUpdate, Functions, Hash, StableFunctions,
};

pub fn detect_changes(
    old_functions: Option<StableFunctions>,
    new_functions: &Functions,
) -> Changes {
    let old_functions = old_functions.unwrap_or_default();

    let mut added = BTreeMap::new();
    let mut updated = Vec::new();

    for new_function in new_functions.named.iter() {
        if old_functions
            .named_by_hash(&Hash::new(&new_function.fragment.inner))
            .is_some()
        {
            // Function has not changed. We can forget about it.
            continue;
        }

        if let Some(old_function) =
            old_functions.named.by_name(&new_function.name)
        {
            // Found a function with the same name. But it can't have the same
            // hash, or we wouldn't have made it here. Assuming the new function
            // is an updated version of the old.
            updated.push(FunctionUpdate {
                old: FunctionInUpdate {
                    location: old_function.location(),
                    function: old_function.inner.clone(),
                },
                new: FunctionInUpdate {
                    location: new_function.location(),
                    function: new_function.inner.clone(),
                },
            });

            continue;
        }

        // If we make it here, there was neither an identical function before,
        // nor one with the same name. This must mean this function is new.
        added.insert(new_function.location(), new_function.inner.clone());
    }

    Changes { added, updated }
}
