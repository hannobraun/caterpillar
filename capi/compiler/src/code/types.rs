use super::{Index, IndexMap};

/// # The types that were inferred in the code
#[derive(Debug, Default)]
pub struct Types {
    pub inner: IndexMap<Type>,
}

/// # The signature of an expression
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
    /// # The inputs that the expression consumes
    pub inputs: Vec<Index<Type>>,

    /// # The outputs that the expression produces
    pub outputs: Vec<Index<Type>>,
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

    /// # A number
    ///
    /// ## Implementation Note
    ///
    /// Since the language is still mostly untyped, this actually covers many
    /// different types that are all represented as a 32-bit number.
    ///
    /// I expect that this will get split into multiple, more specific numeric
    /// types at some point.
    Number,

    /// # The type is unknown
    ///
    /// This is used as a placeholder, while the type is still being inferred.
    /// It can also be the end result, if the type _can't_ be inferred.
    ///
    /// For now, the type system isn't advanced enough to guarantee full
    /// inference. Code that deals with types should be able to deal with this
    /// variant.
    Unknown,
}
