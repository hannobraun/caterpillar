use crate::runtime::{Function, Value};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    PushFrame { function: Function },
    PopFrame,
    DefineBinding { name: String, value: Value },
    PushOperand { operand: Value },
    PopOperand,
}
