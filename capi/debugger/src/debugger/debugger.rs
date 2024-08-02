use capi_process::Value;
use capi_protocol::memory::Memory;

use super::ActiveFunctions;

#[derive(Clone, Debug)]
pub struct Debugger {
    pub active_functions: ActiveFunctions,
    pub operands: Vec<Value>,
    pub memory: Option<Memory>,
}
