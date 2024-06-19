use crate::runtime::{Function, MissingOperand, Value};

pub enum Event<'r> {
    PushFrame {
        function: Function,
    },
    PopFrame,
    DefineBinding {
        name: String,
        value: Value,
    },
    PushOperand {
        operand: Value,
    },
    PopOperand {
        operand: &'r mut Result<Value, MissingOperand>,
    },
}
