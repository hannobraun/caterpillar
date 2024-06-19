use crate::{runtime::DataStack, state::Memory};

use super::ActiveFunctions;

#[derive(Clone)]
pub struct Debugger {
    pub active_functions: ActiveFunctions,
    pub data_stack: Option<DataStack>,
    pub memory: Option<Memory>,
}
