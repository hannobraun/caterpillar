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
        let number = value.expect()?;
        Ok(number)
    }

    fn pop_inner(&mut self, expected: &'static str) -> DataStackResult<Value> {
        self.values.pop().ok_or(DataStackError {
            kind: DataStackErrorKind::StackIsEmpty,
            expected,
        })
    }
}

#[derive(Debug, EnumVariantType)]
#[evt(module = "value")]
pub enum Value {
    Number(i64),
}

impl Value {
    pub fn expect<T>(self) -> DataStackResult<T>
    where
        T: TryFrom<Value>,
        <T as TryFrom<Value>>::Error: fmt::Debug,
    {
        Ok(self.try_into().unwrap())
    }
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
        match &self.kind {
            DataStackErrorKind::StackIsEmpty => {
                write!(f, "Stack is empty")?;
            }
        }

        writeln!(f, " (expected {})", self.expected)
    }
}

#[derive(Debug)]
pub enum DataStackErrorKind {
    StackIsEmpty,
}
