use crate::{process::Memory, runtime::DataStack};

use super::ActiveFunctions;

#[derive(Clone)]
pub struct Debugger {
    pub active_functions: ActiveFunctions,
    pub data_stacks: Option<[DataStack; 2]>,
    pub memory: Option<Memory>,
}
