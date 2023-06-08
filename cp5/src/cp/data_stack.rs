use super::syntax::SyntaxTree;

#[derive(Debug)]
pub struct DataStack {
    values: Vec<Value>,
    marker: usize,
}

impl DataStack {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            marker: 0,
        }
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into())
    }

    pub fn pop_any(&mut self) -> Result<Value, DataStackError> {
        self.values.pop().ok_or(DataStackError::PopFromEmptyStack)
    }

    pub fn pop_array(&mut self) -> Result<Vec<Value>, DataStackError> {
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

    pub fn pop_block(&mut self) -> Result<SyntaxTree, DataStackError> {
        self.pop_specific_type("block", |value| {
            let Value::Block(value) = value else {
                return Err(value);
            };

            Ok(value)
        })
    }

    pub fn pop_string(&mut self) -> Result<String, DataStackError> {
        self.pop_specific_type("string", |value| {
            let Value::String(value) = value else {
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

#[derive(Clone, Debug)]
pub enum Value {
    Array(Vec<Value>),
    Bool(bool),
    Block(SyntaxTree),
    String(String),
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

#[derive(Debug, thiserror::Error)]
pub enum DataStackError {
    #[error("Tried to pop value from empty stack")]
    PopFromEmptyStack,

    #[error("Expected value of type `{expected}` but found {actual:?}")]
    UnexpectedType {
        expected: &'static str,
        actual: Value,
    },
}
