use crate::code::{Index, Type};

use super::types::{InferredType, InferredTypes, Result};

#[derive(Debug)]
pub struct LocalStack {
    inner: Option<Vec<Index<InferredType>>>,
}

impl LocalStack {
    pub fn get(&self) -> Option<&Vec<Index<InferredType>>> {
        self.inner.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut Vec<Index<InferredType>>> {
        self.inner.as_mut()
    }

    pub fn invalidate(&mut self) {
        self.inner = None;
    }
}

impl Default for LocalStack {
    fn default() -> Self {
        Self {
            inner: Some(Vec::new()),
        }
    }
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
