use crate::runtime::{MissingOperand, Value};

pub enum Event<'r> {
    PopOperand {
        value: &'r mut Result<Value, MissingOperand>,
    },
}
