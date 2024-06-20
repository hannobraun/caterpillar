use crate::runtime::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    PushOperand { operand: Value },
    PopOperand,
}
