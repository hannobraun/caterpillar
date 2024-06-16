use crate::{
    process::{Memory, Process},
    syntax,
    updates::Update,
};

use super::model::{ActiveFunctions, Debugger};

pub struct RemoteProcess {
    pub functions: Option<syntax::Functions>,
    pub process: Option<Process>,
    pub memory: Option<Memory>,
}

impl RemoteProcess {
    pub fn new() -> Self {
        Self {
            functions: None,
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
            Update::SourceCode {
                functions,
                source_map,
            } => {
                self.functions = Some(functions);
                let _ = source_map;
            }
        }
    }

    pub fn to_debugger(&self) -> Debugger {
        let active_functions = ActiveFunctions::new(
            self.functions.as_ref(),
            self.process.as_ref().map(|process| &process.source_map),
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
