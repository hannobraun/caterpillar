use std::{slice, vec};

use super::Expressions;

#[derive(Debug)]
pub struct DataStack {
    inner: Vec<Value>,
}

impl DataStack {
    pub fn new() -> Self {
        let inner = Vec::new();
        Self { inner }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn pop_any(&mut self) -> Value {
        self.inner.pop().expect("Stack is empty")
    }

    pub fn pop_block(&mut self) -> Block {
        let Value::Block(value) = self.pop_any() else {
            panic!("Expected block")
        };
        value
    }

    pub fn pop_bool(&mut self) -> bool {
        let Value::Bool(value) = self.pop_any() else {
            panic!("Expected `bool`")
        };
        value
    }

    pub fn pop_list(&mut self) -> Vec<Value> {
        let Value::List(value) = self.pop_any() else {
            panic!("Expected list")
        };
        value
    }

    pub fn pop_u8(&mut self) -> u8 {
        let Value::U8(value) = self.pop_any() else {
            panic!("Expected `u8`")
        };
        value
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.inner.push(value.into())
    }
}

impl IntoIterator for DataStack {
    type Item = Value;
    type IntoIter = vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'r> IntoIterator for &'r DataStack {
    type Item = &'r Value;
    type IntoIter = slice::Iter<'r, Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    Block(Block),
    Bool(bool),
    List(Vec<Value>),
    Name(String),
    U8(u8),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Self::List(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

pub type Block = Expressions;
