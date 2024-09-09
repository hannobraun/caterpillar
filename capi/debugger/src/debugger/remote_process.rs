use capi_game_engine::memory::Memory;
use capi_process::Process;
use capi_protocol::updates::UpdateFromRuntime;

#[derive(Debug, Default)]
pub struct RemoteProcess {
    pub process: Option<Process>,
    pub memory: Option<Memory>,
}

impl RemoteProcess {
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
