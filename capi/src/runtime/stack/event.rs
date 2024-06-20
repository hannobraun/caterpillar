use crate::runtime::{Function, Value};

pub enum Event {
    PushFrame { function: Function },
    PopFrame,
    DefineBinding { name: String, value: Value },
    PushOperand { operand: Value },
    PopOperand,
}
