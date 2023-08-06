use std::fmt;

use enum_variant_type::EnumVariantType;

pub struct DataStack {
    values: Vec<Value>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into())
    }

    pub fn pop_any(&mut self) -> DataStackResult<Value> {
        self.pop_inner("any value")
    }

    pub fn pop_number(&mut self) -> DataStackResult<value::Number> {
        let value = self.pop_inner("number")?;
        Ok(value.try_into().unwrap())
    }

    fn pop_inner(&mut self, expected: &'static str) -> DataStackResult<Value> {
        let value = self.values.pop().ok_or(DataStackError {
            kind: DataStackErrorKind::StackIsEmpty,
            expected,
        })?;

        Ok(value)
    }
}

#[derive(Debug, EnumVariantType)]
#[evt(module = "value")]
pub enum Value {
    Number(i64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{number}"),
        }
    }
}

pub type DataStackResult<T> = Result<T, DataStackError>;

#[derive(Debug, thiserror::Error)]
pub struct DataStackError {
    pub kind: DataStackErrorKind,
    pub expected: &'static str,
}

impl fmt::Display for DataStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            DataStackErrorKind::StackIsEmpty => {
                writeln!(f, "Stack is empty (expected {})", self.expected)
            }
        }
    }
}

#[derive(Debug)]
pub enum DataStackErrorKind {
    StackIsEmpty,
}
