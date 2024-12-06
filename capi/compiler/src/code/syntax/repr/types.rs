/// # The type signature of an expression, as it appears in the syntax
///
/// ## Implementation Note
///
/// This type is a stopgap. The plan is to eventually remove it, once full type
/// inference is supported.
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
    pub inputs: Vec<SyntaxType>,

    /// # The outputs of the expression
    pub outputs: Vec<SyntaxType>,
}

/// # A type, as it appears in the syntax
///
/// This is distinct from a type that has been resolved, which the compiler can
/// reason about.
///
/// ## Implementation Note
///
/// This type is a stopgap. The plan is to eventually remove it, once full type
/// inference is supported.
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
pub enum SyntaxType {
    /// # A function type
    Function {
        /// # The signature of the function
        signature: Signature,
    },

    /// # An identifier that refers to a type
    Identifier { name: String },
}
