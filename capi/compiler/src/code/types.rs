/// # The signature of an expression
pub struct Signature {
    /// # The inputs that the expression consumes
    pub inputs: Vec<Type>,

    /// # The outputs that the expression produces
    pub outputs: Vec<Type>,
}

/// # The type of a value
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
    /// # A function
    Function {
        /// # The function's signature
        signature: Signature,
    },
    Number,
}
