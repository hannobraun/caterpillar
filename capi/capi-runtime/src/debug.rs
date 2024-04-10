use crate::{Function, Functions};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct DebugState {
    pub functions: Vec<Function>,
}

impl DebugState {
    pub fn new(functions: Functions) -> Self {
        Self {
            functions: functions.inner,
        }
    }
}
