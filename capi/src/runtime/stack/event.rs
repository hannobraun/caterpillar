use crate::runtime::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    DefineBinding { name: String, value: Value },
    PushOperand { operand: Value },
    PopOperand,
}
