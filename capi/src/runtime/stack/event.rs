use crate::runtime::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    PopFrame,
    DefineBinding { name: String, value: Value },
    PushOperand { operand: Value },
    PopOperand,
}
