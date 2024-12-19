use std::{collections::BTreeSet, fmt, result};

use crate::code::{syntax::MemberLocation, Index, IndexMap, Type};

#[derive(Debug, Default)]
pub struct InferredTypes {
    pub inner: IndexMap<InferredType>,
    unification: BTreeSet<BTreeSet<Index<InferredType>>>,
}

impl InferredTypes {
    pub fn push(&mut self, type_: InferredType) -> Index<InferredType> {
        self.inner.push(type_)
    }

    #[cfg(test)]
    pub fn unify(&mut self, [a, b]: [&Index<InferredType>; 2]) {
        let relevant_sets = self
            .unification
            .iter()
            .filter(|set| set.contains(a) || set.contains(b))
            .cloned()
            .collect::<Vec<_>>();

        let mut unified_set = BTreeSet::new();
        unified_set.extend([a, b]);

        for set in relevant_sets {
            self.unification.remove(&set);
            unified_set = unified_set.union(&set).cloned().collect();
        }

        self.unification.insert(unified_set);
    }

    pub fn resolve(&self, index: &Index<InferredType>) -> Result<InferredType> {
        let mut resolved = self.get(index).clone();
        let equivalence_set =
            self.unification.iter().find(|set| set.contains(index));

        for other in equivalence_set.into_iter().flatten() {
            let other = self.get(other);
            if let InferredType::Known(_) = other {
                resolved = other.clone();
            }
        }

        Ok(resolved)
    }

    fn get(&self, index: &Index<InferredType>) -> &InferredType {
        let Some(type_) = self.inner.get(index) else {
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

pub type Result<T> = result::Result<T, TypeError>;

#[derive(Debug, Eq, PartialEq)]
pub struct TypeError {
    pub expected: ExpectedType,
    pub actual: Option<Type>,
    pub location: MemberLocation,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExpectedType {
    Function,
    Specific(Type),
    Unknown,
}

impl fmt::Display for ExpectedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Function => write!(f, "function"),
            Self::Specific(type_) => write!(f, "`{type_}`"),
            Self::Unknown => write!(f, "unknown type"),
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

        assert_eq!(types.resolve(&index), Ok(type_));
    }

    #[test]
    fn resolve_unknown() {
        let mut types = InferredTypes::default();

        let type_ = InferredType::Unknown;
        let index = types.push(type_.clone());

        assert_eq!(types.resolve(&index), Ok(type_));
    }

    #[test]
    fn resolve_unified() {
        let mut types = InferredTypes::default();

        let type_ = InferredType::Known(Type::Number);

        let a = types.push(type_.clone());
        let b = types.push(InferredType::Unknown);

        types.unify([&a, &b]);

        assert_eq!(types.resolve(&a).as_ref(), Ok(&type_));
        assert_eq!(types.resolve(&b), Ok(type_));
    }

    #[test]
    fn resolve_unified_with_type_known_only_indirectly() {
        let mut types = InferredTypes::default();

        let type_ = InferredType::Known(Type::Number);

        let a = types.push(type_.clone());
        let b = types.push(InferredType::Unknown);
        let c = types.push(InferredType::Unknown);

        types.unify([&a, &b]);
        types.unify([&b, &c]);

        assert_eq!(types.resolve(&c), Ok(type_));
    }
}
