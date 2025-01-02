use crate::code::{Index, Type};

use super::types::{InferredType, InferredTypes, Result};

#[derive(Debug)]
pub struct MaybeLocalStack {
    inner: Option<LocalStack>,
}

impl MaybeLocalStack {
    pub fn get(&self) -> Option<&Vec<Index<InferredType>>> {
        self.inner.as_ref().map(|local_stack| &local_stack.inner)
    }

    pub fn get_mut(&mut self) -> Option<&mut Vec<Index<InferredType>>> {
        self.inner
            .as_mut()
            .map(|local_stack| &mut local_stack.inner)
    }

    pub fn invalidate(&mut self) {
        self.inner = None;
    }
}

impl Default for MaybeLocalStack {
    fn default() -> Self {
        Self {
            inner: Some(LocalStack::default()),
        }
    }
}

#[derive(Debug, Default)]
pub struct LocalStack {
    pub inner: Vec<Index<InferredType>>,
}

pub fn make_direct(
    local_stack: &[Index<InferredType>],
    types: &mut InferredTypes,
) -> Result<Option<Vec<Type>>> {
    local_stack
        .iter()
        .map(|index| types.resolve(index)?.into_type(types))
        .collect()
}
