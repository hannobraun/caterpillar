use crate::{Function, Functions};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct DebugState {
    pub functions: Vec<Function>,
}

impl DebugState {
    pub fn new(functions: Functions) -> Self {
        let functions = functions.inner;
        Self { functions }
    }
}
