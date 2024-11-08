use std::collections::BTreeMap;

use super::{Function, Index, NamedFunction};

/// # The changes between two versions of code
#[derive(Debug)]
pub struct Changes {
    /// # The functions that were added in the new version
    pub added: BTreeMap<Index<NamedFunction>, Function>,

    /// # The functions that were updated in the new version
    pub updated: Vec<FunctionUpdate>,
}

impl Changes {
    /// # Access the new or updated function with the given index
    ///
    /// Can return `None`, if the function with the given index is neither new
    /// nor updated.
    pub fn new_or_updated_function(
        &self,
        index: &Index<NamedFunction>,
    ) -> Option<&Function> {
        if let Some(function) = self.added.get(index) {
            return Some(function);
        }

        self.updated.iter().find_map(|update| {
            let new = &update.new;
            (new.index == *index).then_some(&new.function)
        })
    }
}

/// # A function update
#[derive(Debug)]
pub struct FunctionUpdate {
    /// # The old version of the function
    pub old: FunctionInUpdate,

    /// # The new version of the function
    pub new: FunctionInUpdate,
}

/// # An function that is part of an update; either an old or a new version
#[derive(Debug)]
pub struct FunctionInUpdate {
    pub index: Index<NamedFunction>,
    pub function: Function,
}
