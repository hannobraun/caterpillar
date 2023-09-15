use crate::language::repr::eval::{
    fragments::FragmentId,
    value::{Type, TypeError, ValueKind},
};

#[derive(Debug)]
pub struct DataStack {
    values: Vec<ValueKind>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: impl Into<ValueKind>) {
        self.values.push(value.into())
    }

    pub fn pop_any(&mut self) -> DataStackResult<ValueKind> {
        self.pop_inner("any value")
    }

    pub fn pop_specific<T: Type>(&mut self) -> DataStackResult<T> {
        let value = self.pop_inner(T::NAME)?;
        let number = value.expect(T::NAME)?;
        Ok(number)
    }

    fn pop_inner(
        &mut self,
        expected: &'static str,
    ) -> DataStackResult<ValueKind> {
        self.values
            .pop()
            .ok_or(DataStackError::StackIsEmpty { expected })
    }

    pub fn replace(&mut self, old: FragmentId, new: FragmentId) {
        for value in &mut self.values {
            if let ValueKind::Block { start } = value {
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
