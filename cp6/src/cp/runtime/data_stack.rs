use core::fmt;

use crate::cp::{pipeline::ir::analyzer_output::AnalyzerOutput, Formatter};

#[derive(Clone, Debug, Default)]
pub struct DataStack {
    values: Vec<Value>,
    marker: usize,
}

impl DataStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into())
    }

    pub fn pop_any(&mut self) -> Result<Value, DataStackError> {
        self.values.pop().ok_or(DataStackError::PopFromEmptyStack)
    }

    pub fn pop_array(&mut self) -> Result<Array, DataStackError> {
        self.pop_specific_type("array", |value| {
            let Value::Array(value) = value else {
                return Err(value);
            };

            Ok(value)
        })
    }

    pub fn pop_bool(&mut self) -> Result<bool, DataStackError> {
        self.pop_specific_type("bool", |value| {
            let Value::Bool(value) = value else {
                return Err(value);
            };

            Ok(value)
        })
    }

    pub fn pop_block(&mut self) -> Result<AnalyzerOutput, DataStackError> {
        self.pop_specific_type("block", |value| {
            let Value::Block(value) = value else {
                return Err(value);
            };

            Ok(value)
        })
    }

    pub fn pop_u8(&mut self) -> Result<u8, DataStackError> {
        self.pop_specific_type("u8", |value| {
            let Value::U8(value) = value else {
                return Err(value);
            };

            Ok(value)
        })
    }

    fn pop_specific_type<T>(
        &mut self,
        expected: &'static str,
        f: impl FnOnce(Value) -> Result<T, Value>,
    ) -> Result<T, DataStackError> {
        let value = self.pop_any()?;
        f(value).map_err(|value| DataStackError::UnexpectedType {
            expected,
            actual: value,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn mark(&mut self) {
        self.marker = self.values.len();
    }

    pub fn drain_values_from_marker(
        &mut self,
    ) -> impl Iterator<Item = Value> + '_ {
        let index = usize::min(self.marker, self.values.len());
        self.values.drain(index..)
    }
}

impl fmt::Display for DataStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, value) in self.values.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }

            write!(f, "{value}")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Array(Array),
    Bool(bool),
    Block(AnalyzerOutput),
    String(String),
    U8(u8),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Array(value) => write!(f, "{value}"),
            Value::Bool(value) => write!(f, "{value}"),
            Value::Block(value) => write!(f, "{{ {} }}", Formatter(value)),
            Value::String(value) => write!(f, "{value}"),
            Value::U8(value) => write!(f, "{value}"),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Array {
    pub elements: Vec<Value>,
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        if !self.elements.is_empty() {
            write!(f, " ")?;
        }

        for element in &self.elements {
            write!(f, "{element} ")?;
        }

        write!(f, "]")?;

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum DataStackError {
    #[error("Tried to pop value from empty stack")]
    PopFromEmptyStack,

    #[error("Expected value of type `{expected}` but found {actual:?}")]
    UnexpectedType {
        expected: &'static str,
        actual: Value,
    },
}
