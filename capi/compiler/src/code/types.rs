use super::Index;

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
        signature: ConcreteSignature,
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
}

/// # The signature of an expression
#[derive(
    Clone,
    Debug,
    Default,
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

/// # A concrete signature
///
/// Most code should use `Signature` instead, which references into signatures
/// stored in `Types`.
///
/// This type is only intended for type signatures that have a lifetime
/// extending beyond that of a running compiler. Like those of intrinsic or host
/// functions.
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
pub struct ConcreteSignature {
    /// # The inputs that the function consumes
    pub inputs: Vec<Type>,

    /// # The outputs that the function produces
    pub outputs: Vec<Type>,
}

impl<I, O> From<(I, O)> for ConcreteSignature
where
    I: IntoIterator<Item = Type>,
    O: IntoIterator<Item = Type>,
{
    fn from((inputs, outputs): (I, O)) -> Self {
        Self {
            inputs: inputs.into_iter().collect(),
            outputs: outputs.into_iter().collect(),
        }
    }
}
