use capi_game_engine::memory::Memory;
use capi_process::{Process, Value};

use super::{ActiveFunctions, DebugCode};

#[derive(Clone, Debug, Default)]
pub struct Debugger {
    pub code: DebugCode,
    pub memory: Option<Memory>,

    pub active_functions: ActiveFunctions,
    pub operands: Vec<Value>,
}

impl Debugger {
    pub fn update(&mut self, process: Option<&Process>) {
        self.active_functions =
            ActiveFunctions::new(self.code.code_from_server.as_ref(), process);
        self.operands = process
            .map(|process| {
                process
                    .stack()
                    .operands_in_current_stack_frame()
                    .copied()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
    }
}
