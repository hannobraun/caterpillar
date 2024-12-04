use crate::code::Type;

use super::expression::Expression;

/// # An expression, optionally annotated with a type
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct AnnotatedExpression {
    /// # The expression
    pub inner: Expression,

    /// # The optional type annotation that applies to the expression
    pub signature: Option<Signature>,
}

/// # The type signature of an expression
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct Signature {
    /// # The inputs of the expression
    pub inputs: Vec<Type>,

    /// # The outputs of the expression
    pub outputs: Vec<Type>,
}
