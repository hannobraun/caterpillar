use crate::code::Type;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InferredType {
    Known(Type),
    Unknown,
}

impl InferredType {
    pub fn unknown() -> Self {
        Self::Unknown
    }

    pub fn into_type(self) -> Option<Type> {
        match self {
            Self::Known(type_) => Some(type_),
            Self::Unknown { .. } => None,
        }
    }
}
