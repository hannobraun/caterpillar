use crate::{process::Process, runtime::DataStack};

use super::ActiveFunctions;

#[derive(Clone)]
pub struct Debugger {
    pub active_functions: ActiveFunctions,
    pub data_stacks: Option<[DataStack; 2]>,
    pub process: Option<Process>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            active_functions: ActiveFunctions::new(None),
            data_stacks: None,
            process: None,
        }
    }

    pub fn update_from_process(&mut self, process: Process) {
        self.active_functions = ActiveFunctions::new(Some(&process));
        self.data_stacks = Some([
            process.previous_data_stack.clone(),
            process.evaluator.data_stack().clone(),
        ]);
        self.process = Some(process);
    }
}
