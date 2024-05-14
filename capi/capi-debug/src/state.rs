use capi_runtime::{Function, ProgramState};

pub struct ExecutionContext {
    pub state: ProgramState,
    pub function: Function,
}
