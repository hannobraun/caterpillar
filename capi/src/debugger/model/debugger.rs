use crate::{process::Memory, runtime::DataStack, updates::Update};

use super::ActiveFunctions;

#[derive(Clone)]
pub struct Debugger {
    pub active_functions: ActiveFunctions,
    pub data_stacks: Option<[DataStack; 2]>,
    pub memory: Option<Memory>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            active_functions: ActiveFunctions::new(None),
            data_stacks: None,
            memory: None,
        }
    }

    pub fn on_update(&mut self, update: Update) {
        match update {
            Update::Memory { memory } => {
                self.memory = Some(memory);
            }
            Update::Process(process) => {
                self.active_functions = ActiveFunctions::new(Some(&process));
                self.data_stacks = Some([
                    process.previous_data_stack,
                    process.evaluator.data_stack().clone(),
                ]);
            }
        }
    }
}
