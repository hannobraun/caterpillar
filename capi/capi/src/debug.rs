use capi_runtime::Functions;

pub struct DebugState {
    pub functions: Functions,
}

impl DebugState {
    pub fn new(functions: Functions) -> Self {
        Self { functions }
    }
}
