use std::collections::BTreeSet;

use crate::code::{Index, Type};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InferredType {
    Known(Type),
    Unknown {
        equal_to: BTreeSet<Index<InferredType>>,
    },
}

impl InferredType {
    pub fn unknown() -> Self {
        Self::Unknown {
            equal_to: BTreeSet::new(),
        }
    }

    pub fn into_type(self) -> Option<Type> {
        match self {
            Self::Known(type_) => Some(type_),
            Self::Unknown { .. } => None,
        }
    }
}
