use capi_process::Operands;

use crate::state::Memory;

use super::ActiveFunctions;

#[derive(Clone)]
pub struct Debugger {
    pub active_functions: ActiveFunctions,
    pub operands: Option<Operands>,
    pub memory: Option<Memory>,
}
