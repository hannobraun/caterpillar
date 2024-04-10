use capi_runtime::{Function, Functions};

#[derive(Clone)]
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
