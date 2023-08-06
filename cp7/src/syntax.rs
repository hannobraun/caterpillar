use std::collections::HashMap;

use crate::value::Value;

pub struct Syntax {
    pub inner: HashMap<SyntaxHandle, SyntaxFragment>,
}

impl Syntax {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, fragment: SyntaxFragment) -> SyntaxHandle {
        // This is a placeholder. Eventually, we need to add a hash that
        // uniquely addresses the fragment.
        let handle = SyntaxHandle {};
        self.inner.insert(handle, fragment);
        handle
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct SyntaxHandle {}

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    pub elements: Vec<SyntaxFragment>,
}

impl SyntaxTree {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SyntaxFragment {
    pub payload: SyntaxElement,
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    FnRef(String),
    Value(Value),
}
