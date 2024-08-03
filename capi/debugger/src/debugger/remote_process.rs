use capi_game_engine::{host::GameEngineHost, memory::Memory};
use capi_process::Process;
use capi_protocol::updates::{Code, Update};

use super::{ActiveFunctions, Debugger};

#[derive(Debug, Default)]
pub struct RemoteProcess {
    pub code: Option<Code>,
    pub process: Option<Process<GameEngineHost>>,
    pub memory: Option<Memory>,
}

impl RemoteProcess {
    pub fn on_code_update(&mut self, code: Code) {
        self.code = Some(code);
    }

    pub fn on_runtime_update(&mut self, update: Update<GameEngineHost>) {
        match update {
            Update::Memory { memory } => {
                self.memory = Some(memory);
            }
            Update::Process(process) => {
                self.process = Some(process);
            }
        }
    }

    pub fn to_debugger(&self) -> Debugger {
        let active_functions =
            ActiveFunctions::new(self.code.as_ref(), self.process.as_ref());
        let operands = self
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
        let memory = self.memory.clone();

        Debugger {
            active_functions,
            operands,
            memory,
        }
    }
}
