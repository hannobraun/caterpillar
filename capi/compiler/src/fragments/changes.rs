use std::collections::BTreeMap;

use super::{Function, FunctionIndexInRootContext};

/// # The changes between two versions of code
#[derive(Debug)]
pub struct Changes {
    /// # The functions that were added in the new version
    pub added: BTreeMap<FunctionIndexInRootContext, Function>,

    /// # The functions that were updated in the new version
    pub updated: Vec<FunctionUpdate>,
}

/// # A function update
#[derive(Debug)]
pub struct FunctionUpdate {
    /// # The old version of the function
    pub old: (FunctionIndexInRootContext, Function),

    /// # The new version of the function
    pub new: (FunctionIndexInRootContext, Function),
}
