use capi_runtime::Functions;

#[derive(Clone)]
pub struct DebugState {
    pub functions: Functions,
}

impl DebugState {
    pub fn new(functions: Functions) -> Self {
        Self { functions }
    }
}
