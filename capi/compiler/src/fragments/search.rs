//! # Types related to searching code
//!
//! Contains types that are returned by searches in code. Those types themselves
//! provide more convenient functionality for searching within them.

use super::{Function, FunctionLocation};

/// # A function that was found by a search
pub struct FoundFunction<'r> {
    /// # The function that was found
    pub function: &'r Function,

    /// # The location of the function that was found
    pub location: &'r FunctionLocation,
}
