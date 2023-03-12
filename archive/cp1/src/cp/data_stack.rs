use std::{fmt, slice, vec};

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

    pub fn pop_list(&mut self) -> List {
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

impl fmt::Display for DataStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, value) in self.into_iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{value}")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Value {
    Block(Block),
    Bool(bool),
    List(List),
    Name(String),
    U8(u8),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Block(block) => write!(f, "{block}"),
            Value::Bool(bool) => write!(f, "{bool}"),
            Value::List(list) => write!(f, "{list}"),
            Value::Name(name) => write!(f, ":{name}"),
            Value::U8(u8) => write!(f, "{u8}"),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<List> for Value {
    fn from(value: List) -> Self {
        Self::List(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Block {
    pub expressions: Expressions,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {} }}", self.expressions)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct List {
    pub values: Vec<Value>,
}

impl FromIterator<Value> for List {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self {
            values: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for List {
    type Item = Value;

    type IntoIter = vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for value in &self.values {
            write!(f, " {value}")?;
        }
        write!(f, " ]")?;

        Ok(())
    }
}
