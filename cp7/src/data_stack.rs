use std::fmt;

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
        let Value::Number(number) = value;
        Ok(number)
    }

    fn pop_inner(&mut self, expected: &'static str) -> DataStackResult<Value> {
        self.values.pop().ok_or(DataStackError {
            kind: DataStackErrorKind::StackIsEmpty,
            expected,
        })
    }
}

#[derive(Debug)]
pub enum Value {
    Number(value::Number),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{number}"),
        }
    }
}

impl From<value::Number> for Value {
    fn from(number: value::Number) -> Self {
        Self::Number(number)
    }
}

pub mod value {
    pub type Number = i64;
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
