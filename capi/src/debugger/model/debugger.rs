use crate::process::Process;

#[derive(Clone)]
pub struct Debugger {
    pub process: Option<Process>,
}
