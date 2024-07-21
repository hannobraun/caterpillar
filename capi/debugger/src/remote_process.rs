use capi_process::{GameEngineHost, Process};
use capi_protocol::{
    memory::Memory,
    update::{SourceCode, Update},
};

use super::model::{ActiveFunctions, Debugger};

#[derive(Default)]
pub struct RemoteProcess {
    pub source_code: Option<SourceCode>,
    pub process: Option<Process<GameEngineHost>>,
    pub memory: Option<Memory>,
}

impl RemoteProcess {
    pub fn on_update(&mut self, update: Update) {
        match update {
            Update::Memory { memory } => {
                self.memory = Some(memory);
            }
            Update::Process(process) => {
                self.process = Some(process);
            }
            Update::SourceCode(source_code) => {
                self.source_code = Some(source_code);
            }
        }
    }

    pub fn to_debugger(&self) -> Debugger {
        let active_functions = ActiveFunctions::new(
            self.source_code.as_ref(),
            self.process.as_ref(),
        );
        let operands = self
            .process
            .as_ref()
            .map(|process| process.stack().operands().unwrap().clone());
        let memory = self.memory.clone();

        Debugger {
            active_functions,
            operands,
            memory,
        }
    }
}
