use capi_game_engine::memory::Memory;
use capi_process::{Process, Value};
use capi_protocol::updates::UpdateFromRuntime;

use super::{ActiveFunctions, DebugCode};

#[derive(Clone, Debug, Default)]
pub struct Debugger {
    pub code: DebugCode,
    pub process: Option<Process>,
    pub memory: Option<Memory>,

    pub active_functions: ActiveFunctions,
    pub operands: Vec<Value>,
}

impl Debugger {
    pub fn on_update_from_runtime(&mut self, update: UpdateFromRuntime) {
        match update {
            UpdateFromRuntime::Memory { memory } => {
                self.memory = Some(memory);
            }
            UpdateFromRuntime::Process(process) => {
                self.process = Some(process);
            }
        }
    }

    pub fn update(&mut self) {
        self.active_functions = ActiveFunctions::new(
            self.code.code_from_server.as_ref(),
            self.process.as_ref(),
        );
        self.operands = self
            .process
            .as_ref()
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
