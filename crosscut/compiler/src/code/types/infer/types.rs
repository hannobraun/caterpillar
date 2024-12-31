use std::{collections::BTreeSet, fmt, result};

use crate::code::{syntax::MemberLocation, Index, IndexMap, Type};

#[derive(Debug, Default)]
pub struct InferredTypes {
    inner: IndexMap<InferredType>,
    equivalence_sets: BTreeSet<BTreeSet<Index<InferredType>>>,
}

impl InferredTypes {
    pub fn push(&mut self, type_: InferredType) -> Index<InferredType> {
        self.inner.push(type_)
    }

    pub fn unify(&mut self, [a, b]: [&Index<InferredType>; 2]) {
        let relevant_sets = self
            .equivalence_sets
            .iter()
            .filter(|set| set.contains(a) || set.contains(b))
            .cloned()
            .collect::<Vec<_>>();

        let mut unified_set = BTreeSet::new();
        unified_set.extend([a, b]);

        for set in relevant_sets {
            self.equivalence_sets.remove(&set);
            unified_set = unified_set.union(&set).cloned().collect();
        }

        self.equivalence_sets.insert(unified_set);
    }

    pub fn resolve(&self, index: &Index<InferredType>) -> Result<InferredType> {
        let mut resolved = self.get(index).clone();
        let equivalence_set =
            self.equivalence_sets.iter().find(|set| set.contains(index));

        for other in equivalence_set.into_iter().flatten() {
            let other = self.get(other);
            resolved = merge_inferred_types([resolved, other.clone()])?;
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
}

fn merge_inferred_types([a, b]: [InferredType; 2]) -> Result<InferredType> {
    let type_ = match (a, b) {
        (InferredType::Direct(a), InferredType::Direct(b)) => {
            merge_direct_types([a, b])?
        }
        (InferredType::Unknown, b) => b,
        (a, InferredType::Unknown) => {
            // Other type doesn't add any new information.
            a
        }
    };

    Ok(type_)
}

fn merge_direct_types([a, b]: [Type; 2]) -> Result<InferredType> {
    if a == b {
        Ok(InferredType::Direct(a))
    } else {
        Err(TypeError {
            expected: ExpectedType::Specific(a),
            actual: Some(b),
            location: None,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InferredType {
    Direct(Type),
    Unknown,
}

impl InferredType {
    pub fn into_type(self, _: &InferredTypes) -> Result<Option<Type>> {
        let type_ = match self {
            Self::Direct(type_) => Some(type_),
            Self::Unknown { .. } => None,
        };

        Ok(type_)
    }

    pub fn into_expected_type(self) -> ExpectedType {
        match self {
            InferredType::Direct(type_) => ExpectedType::Specific(type_),
            InferredType::Unknown => ExpectedType::Unknown,
        }
    }
}

pub type Result<T> = result::Result<T, TypeError>;

#[derive(Debug, Eq, PartialEq)]
pub struct TypeError {
    pub expected: ExpectedType,
    pub actual: Option<Type>,
    pub location: Option<MemberLocation>,
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
    use crate::code::{
        types::infer::types::{ExpectedType, InferredType, TypeError},
        Signature, Type,
    };

    use super::InferredTypes;

    #[test]
    fn resolve_known() {
        let mut types = InferredTypes::default();

        let type_ = InferredType::Direct(Type::Number);
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

        let type_ = InferredType::Direct(Type::Number);

        let a = types.push(type_.clone());
        let b = types.push(InferredType::Unknown);

        types.unify([&a, &b]);

        assert_eq!(types.resolve(&a).as_ref(), Ok(&type_));
        assert_eq!(types.resolve(&b), Ok(type_));
    }

    #[test]
    fn resolve_unified_with_type_known_only_indirectly() {
        let mut types = InferredTypes::default();

        let type_ = InferredType::Direct(Type::Number);

        let a = types.push(type_.clone());
        let b = types.push(InferredType::Unknown);
        let c = types.push(InferredType::Unknown);

        types.unify([&a, &b]);
        types.unify([&b, &c]);

        assert_eq!(types.resolve(&c), Ok(type_));
    }

    #[test]
    fn resolve_conflicting_unified() {
        let mut types = InferredTypes::default();

        let a = Type::Number;
        let b = Type::Function {
            signature: Signature {
                inputs: vec![],
                outputs: vec![Type::Number],
            },
        };

        let index_a = types.push(InferredType::Direct(a.clone()));
        let index_b = types.push(InferredType::Direct(b.clone()));

        types.unify([&index_a, &index_b]);

        assert_eq!(
            types.resolve(&index_a),
            Err(TypeError {
                expected: ExpectedType::Specific(a.clone()),
                actual: Some(b.clone()),
                location: None,
            })
        );
        assert_eq!(
            types.resolve(&index_b),
            Err(TypeError {
                expected: ExpectedType::Specific(b),
                actual: Some(a),
                location: None,
            })
        );
    }
}
