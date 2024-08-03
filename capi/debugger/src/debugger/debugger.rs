use capi_game_engine::memory::Memory;
use capi_process::Value;

use super::ActiveFunctions;

#[derive(Clone, Debug)]
pub struct Debugger {
    pub active_functions: ActiveFunctions,
    pub operands: Vec<Value>,
    pub memory: Option<Memory>,
}
