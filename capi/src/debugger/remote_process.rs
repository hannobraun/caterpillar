use crate::{
    process::{Memory, Process},
    updates::Update,
};

use super::model::{ActiveFunctions, Debugger};

pub struct RemoteProcess {
    pub process: Option<Process>,
    pub memory: Option<Memory>,
}

impl RemoteProcess {
    pub fn new() -> Self {
        Self {
            process: None,
            memory: None,
        }
    }

    pub fn on_update(&mut self, update: Update) {
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
        let active_functions = ActiveFunctions::new(
            self.process.as_ref().map(|process| &process.functions),
            self.process.as_ref(),
        );
        let data_stacks = self.process.as_ref().map(|process| {
            [
                process.previous_data_stack.clone(),
                process.evaluator.data_stack().clone(),
            ]
        });
        let memory = self.memory.clone();

        Debugger {
            active_functions,
            data_stacks,
            memory,
        }
    }
}
