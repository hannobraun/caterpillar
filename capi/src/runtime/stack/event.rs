use crate::runtime::{Function, MissingOperand, Value};

pub enum Event<'r> {
    PushFrame {
        function: Function,
    },
    PopOperand {
        value: &'r mut Result<Value, MissingOperand>,
    },
}
