use crate::code::Index;

use super::types::InferredType;

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
