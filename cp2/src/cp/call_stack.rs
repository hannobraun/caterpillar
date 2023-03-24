use std::collections::BTreeMap;

use super::{data_stack::Value, Expressions};

pub struct StackFrame {
    pub bindings: Bindings,
    pub functions: Functions,
}

pub type Bindings = BTreeMap<String, Value>;
pub type Functions = BTreeMap<String, Expressions>;
