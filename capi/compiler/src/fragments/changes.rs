use super::Function;

/// # The changes between two versions of code
#[derive(Debug)]
pub struct Changes {
    /// # The functions that were added in the new version
    pub added: Vec<Function>,

    /// # The functions that were updated in the new version
    pub updated: Vec<UpdatedFunction>,
}

/// # A function that was updated in a new version of the code
#[derive(Debug)]
pub struct UpdatedFunction {
    /// # The old version of the function
    pub old: Function,

    /// # The new version of the function
    pub new: Function,
}