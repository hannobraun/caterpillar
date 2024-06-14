use crate::process::Process;

use super::ActiveFunctions;

#[derive(Clone)]
pub struct Debugger {
    pub active_functions: ActiveFunctions,
    pub process: Option<Process>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            active_functions: ActiveFunctions::new(None),
            process: None,
        }
    }

    pub fn update_from_process(&mut self, process: Process) {
        self.active_functions = ActiveFunctions::new(Some(&process));
        self.process = Some(process);
    }
}
