use crate::process::Process;

#[derive(Clone)]
pub struct Debugger {
    pub process: Option<Process>,
}

impl Debugger {
    pub fn new() -> Self {
        Self { process: None }
    }
}
