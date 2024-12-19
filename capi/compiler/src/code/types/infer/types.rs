use std::collections::BTreeSet;

use crate::code::{Index, IndexMap, Type};

#[derive(Debug, Default)]
pub struct InferredTypes {
    pub inner: IndexMap<InferredType>,
}

impl InferredTypes {
    pub fn push(&mut self, type_: InferredType) -> Index<InferredType> {
        self.inner.push(type_)
    }

    pub fn resolve(&self, index: &Index<InferredType>) -> InferredType {
        self.get(index)
    }

    fn get(&self, index: &Index<InferredType>) -> InferredType {
        let Some(type_) = self.inner.get(index).cloned() else {
            unreachable!(
                "We are never removing any inferred types. Thus, an index can \
                only be invalid, if the caller is mixing indices between \
                multiple instances of this struct.\n\
                \n\
                Since this is a private type, that is always going to be a bug."
            );
        };

        type_
    }

    pub fn unify2(&mut self, types: BTreeSet<Index<InferredType>>) {
        let mut known_types = BTreeSet::new();

        for index in &types {
            if let Some(type_) = self.resolve(index).into_type() {
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

#[cfg(test)]
mod tests {
    use crate::code::{types::infer::types::InferredType, Type};

    use super::InferredTypes;

    #[test]
    fn resolve_known() {
        let mut types = InferredTypes::default();

        let type_ = InferredType::Known(Type::Number);
        let index = types.push(type_.clone());

        assert_eq!(types.resolve(&index), type_);
    }

    #[test]
    fn resolve_unknown() {
        let mut types = InferredTypes::default();

        let type_ = InferredType::Unknown;
        let index = types.push(type_.clone());

        assert_eq!(types.resolve(&index), type_);
    }
}
