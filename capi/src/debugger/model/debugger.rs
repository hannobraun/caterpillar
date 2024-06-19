use crate::{runtime::Operands, state::Memory};

use super::ActiveFunctions;

#[derive(Clone)]
pub struct Debugger {
    pub active_functions: ActiveFunctions,
    pub data_stack: Option<Operands>,
    pub memory: Option<Memory>,
}
