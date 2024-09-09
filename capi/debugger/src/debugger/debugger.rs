use capi_game_engine::memory::Memory;
use capi_process::{Breakpoints, Process, Value};
use capi_protocol::updates::Code;

use super::ActiveFunctions;

#[derive(Clone, Debug, Default)]
pub struct Debugger {
    pub breakpoints: Breakpoints,
    pub active_functions: ActiveFunctions,
    pub operands: Vec<Value>,
    pub memory: Option<Memory>,
}

impl Debugger {
    pub fn update(
        &mut self,
        code: Option<Code>,
        memory: Option<Memory>,
        process: Option<&Process>,
    ) {
        self.active_functions = ActiveFunctions::new(code.as_ref(), process);
        self.operands = process
            .map(|process| {
                process
                    .stack()
                    .operands_in_current_stack_frame()
                    .copied()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        self.memory = memory;
    }
}
