use super::Function;

/// # The changes between two versions of code
#[derive(Debug)]
pub struct Changes {
    /// # The functions that were added in the new version
    pub added: Vec<Function>,

    /// # The functions that were updated in the new version
    pub updated: Vec<(Function, Function)>,
}
