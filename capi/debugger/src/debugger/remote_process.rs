use capi_game_engine::memory::Memory;
use capi_process::Process;
use capi_protocol::updates::{Code, UpdateFromRuntime};

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
}
