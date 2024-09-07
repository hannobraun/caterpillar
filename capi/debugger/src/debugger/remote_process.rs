use capi_game_engine::memory::Memory;
use capi_process::Process;
use capi_protocol::updates::{Code, Update};

use super::Debugger;

#[derive(Debug, Default)]
pub struct RemoteProcess {
    pub code: Option<Code>,
    pub process: Option<Process>,
    pub memory: Option<Memory>,
}

impl RemoteProcess {
    pub fn on_code_update(&mut self, code: Code) {
        self.code = Some(code);
    }

    pub fn on_runtime_update(&mut self, update: Update) {
        match update {
            Update::Memory { memory } => {
                self.memory = Some(memory);
            }
            Update::Process(process) => {
                self.process = Some(process);
            }
        }
    }

    pub fn update_debugger(&self, debugger: &mut Debugger) {
        *debugger = Debugger::new(
            self.code.clone(),
            self.memory.clone(),
            self.process.as_ref(),
        );
    }
}
