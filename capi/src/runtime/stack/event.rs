use crate::runtime::{Function, MissingOperand, Value};

pub enum Event<'r> {
    PushFrame {
        function: Function,
    },
    DefineBinding {
        name: String,
        value: Value,
    },
    PopOperand {
        value: &'r mut Result<Value, MissingOperand>,
    },
}
