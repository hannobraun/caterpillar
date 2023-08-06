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
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct SyntaxHandle {}

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    pub elements: Vec<SyntaxElement>,
}

impl SyntaxTree {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

pub struct SyntaxFragment {
    pub kind: SyntaxElement,
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    FnRef(String),
    Value(Value),
}
