use capi_game_engine::memory::Memory;
use capi_process::{Process, Value};
use capi_protocol::updates::Code;

use super::ActiveFunctions;

#[derive(Clone, Debug, Default)]
pub struct Debugger {
    pub active_functions: ActiveFunctions,
    pub operands: Vec<Value>,
    pub memory: Option<Memory>,
}

impl Debugger {
    pub fn new(
        code: Option<Code>,
        memory: Option<Memory>,
        process: Option<&Process>,
    ) -> Self {
        let active_functions = ActiveFunctions::new(code.as_ref(), process);
        let operands = process
            .map(|process| {
                process
                    .stack()
                    .operands_in_current_stack_frame()
                    .copied()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        Self {
            active_functions,
            operands,
            memory,
        }
    }
}
