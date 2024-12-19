use std::collections::BTreeSet;

use crate::code::{Index, IndexMap, Type};

#[derive(Debug, Default)]
pub struct LocalTypes {
    pub inner: IndexMap<InferredType>,
}

impl LocalTypes {
    pub fn push(&mut self, type_: InferredType) -> Index<InferredType> {
        self.inner.push(type_)
    }

    pub fn get(&self, index: &Index<InferredType>) -> &InferredType {
        let Some(type_) = self.inner.get(index) else {
            unreachable!(
                "We're never removing any local types. Any index must be valid."
            );
        };

        type_
    }

    pub fn unify(&mut self, types: BTreeSet<Index<InferredType>>) {
        let mut known_types = BTreeSet::new();

        for index in &types {
            if let Some(type_) = self.get(index).clone().into_type() {
                known_types.insert(type_);
            }
        }

        if known_types.len() > 1 {
            panic!("Conflicting types: {known_types:?}");
        }

        if let Some(type_) = known_types.into_iter().next() {
            for index in types {
                self.inner.insert(index, InferredType::Known(type_.clone()));
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InferredType {
    Known(Type),
    Unknown,
}

impl InferredType {
    pub fn into_type(self) -> Option<Type> {
        match self {
            Self::Known(type_) => Some(type_),
            Self::Unknown { .. } => None,
        }
    }
}
