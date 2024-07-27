use capi_process::Operands;
use capi_protocol::memory::Memory;

use super::ActiveFunctions;

#[derive(Clone, Debug)]
pub struct Debugger {
    pub active_functions: ActiveFunctions,
    pub operands: Option<Operands>,
    pub memory: Option<Memory>,
}
