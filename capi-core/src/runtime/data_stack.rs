use crate::repr::eval::{
    fragments::FragmentId,
    value::{Type, TypeError, Value, ValueKind},
};

#[derive(Debug, Default)]
pub struct DataStack {
    values: Vec<Value>,
}

impl DataStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value)
    }

    pub fn push_bare(&mut self, kind: impl Into<ValueKind>) {
        let value = Value {
            kind: kind.into(),
            fragment: None,
        };
        self.push(value)
    }

    pub fn pop_any(&mut self) -> DataStackResult<Value> {
        self.pop_inner("any value")
    }

    pub fn pop_specific<T: Type>(
        &mut self,
    ) -> DataStackResult<(T, Option<FragmentId>)> {
        let value = self.pop_inner(T::NAME)?;
        let bare = value.kind.expect(T::NAME)?;
        Ok((bare, value.fragment))
    }

    fn pop_inner(&mut self, expected: &'static str) -> DataStackResult<Value> {
        self.values
            .pop()
            .ok_or(DataStackError::StackIsEmpty { expected })
    }

    pub fn replace(&mut self, old: FragmentId, new: FragmentId) {
        for value in &mut self.values {
            if let ValueKind::Block { start } = &mut value.kind {
                if *start == old {
                    *start = new;
                }
            }
        }
    }
}

pub type DataStackResult<T> = Result<T, DataStackError>;

#[derive(Debug, thiserror::Error)]
pub enum DataStackError {
    #[error("Stack is empty (expected {expected})")]
    StackIsEmpty { expected: &'static str },

    #[error("Unexpected value")]
    UnexpectedValue(#[from] Box<TypeError>),
}
