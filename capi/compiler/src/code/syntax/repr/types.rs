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

/// # A type, as it appears in the syntax
///
/// This is distinct from a type that has been resolved, and that the compiler
/// can reason about.
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
pub enum Type {
    /// # A function type
    Function {
        /// # The signature of the function
        signature: Signature,
    },

    /// # An identifier that refers to a type
    Identifier { name: String },
}
