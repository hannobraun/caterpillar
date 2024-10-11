use std::collections::BTreeMap;

use crate::{
    fragments::{Changes, Fragments, FunctionUpdate},
    hash::Hash,
};

pub fn detect_changes(old: Option<&Fragments>, new: &Fragments) -> Changes {
    let mut old = old
        .map(|fragments| fragments.functions.clone())
        .unwrap_or_default();
    let mut new = new.functions.clone();

    let mut added = BTreeMap::new();
    let mut updated = Vec::new();

    while let Some((new_index, new_function)) = new.pop_first() {
        // We've removed `new_function` from `new`. From here on, where we
        // remove functions from `old`, we don't have to do the same for `new`.

        let same_hash = old.iter().find_map(|(&index, old_function)| {
            if Hash::new(old_function) == Hash::new(&new_function) {
                Some(index)
            } else {
                None
            }
        });
        if let Some(old_index) = same_hash {
            // Function has not changed. We can forget about it.
            old.remove(&old_index);

            continue;
        }

        let same_name = old.iter().find_map(|(&index, old_function)| {
            assert!(
                old_function.name.is_some(),
                "Named function should have a name."
            );
            assert!(
                new_function.name.is_some(),
                "Named function should have a name."
            );

            if old_function.name == new_function.name {
                Some(index)
            } else {
                None
            }
        });
        if let Some(old_index) = same_name {
            // Found a function with the same name. But it can't have the same
            // hash, or we wouldn't have made it here. Assuming the new function
            // is an updated version of the old.
            let old_function = old.remove(&old_index).expect(
                "Just found index in map; expecting it to still be there.",
            );
            updated.push(FunctionUpdate {
                old: (old_index, old_function),
                new: (new_index, new_function),
            });

            continue;
        }

        // If we make it here, there was neither an identical function before,
        // nor one with the same name. This must mean this function is new.
        added.insert(new_index, new_function);
    }

    Changes { added, updated }
}
