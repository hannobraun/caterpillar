use std::collections::BTreeMap;

use super::values::Value;

pub struct CallStack;

impl CallStack {
    pub fn new_stack_frame(&mut self) -> StackFrame {
        StackFrame::new()
    }
}

pub struct StackFrame {
    pub bindings: Bindings,
}

impl StackFrame {
    pub fn new() -> Self {
        Self {
            bindings: Bindings::new(),
        }
    }
}

pub type Bindings = BTreeMap<String, Value>;
