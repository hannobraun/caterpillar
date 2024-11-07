use std::collections::BTreeMap;

use super::{
    BranchLocation, ExpressionLocation, FunctionLocation, Index, IndexMap,
};

/// # The types that were inferred in the code
///
/// ## Separation of Types from `Function`
///
/// This stores not only the types themselves, but associates them with
/// expressions, branches, and functions. It is necessary to store the types
/// separately like this.
///
/// When type inference happens, functions must have been resolved already. If
/// the inferred types were then stored within `Function` (which includes
/// `Branch` and `Expression`), the hash of the function would change, making
/// the previous function resolution invalid.
///
/// This might raise the question, whether it's okay to exclude the types from
/// the hash. And the answer is yes, that's perfectly fine. Type inference is
/// deterministic. It can only change if anything else about the function (which
/// would be included in the hash) changes first.
///
/// ## Implementation Note
///
/// This type refers to expressions, branches, and hashes by location. This is a
/// bit dubious, as a code update could put a different function in that
/// location, and then it wouldn't be possible to have type information for both
/// the old and new versions of the function.
///
/// Right now, this is fine. All calls to a function are updated when the
/// function is updated. So it's not necessary to keep type information for old
/// versions of functions.
///
/// But eventually, it will be possible for code to still call old versions of
/// functions (so refactorings can happen in smaller steps, for example). Then
/// referring to functions by location is no longer any good.
///
/// For now, referring to functions by hash here would cause too many problems
/// though. While referring to a function itself by hash should be easy, we'd
/// also need equivalents of `BranchLocation` and `ExpressionLocation` that
/// refer to the function by hash instead of location.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Types {
    pub inner: IndexMap<Type>,

    pub of_expressions: BTreeMap<ExpressionLocation, Signature>,
    pub of_branches: BTreeMap<BranchLocation, Signature>,
    pub of_functions: BTreeMap<FunctionLocation, Signature>,
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
    /// # The type that has no values
    Empty,

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

impl Signature {
    /// # Convert this signature to a [`ConcreteSignature`]
    ///
    /// Can return `None`, if the indices in this signature are not available in
    /// the provided [`Types`] instance.
    pub fn to_concrete_signature(
        &self,
        types: &Types,
    ) -> Option<ConcreteSignature> {
        let Self { inputs, outputs } = self;

        // This would be a case for `try_map` instead of `map`, but alas, that's
        // not stable.
        let [inputs, outputs] = [inputs, outputs].map(|indices| {
            // This does a weird conversion from `Option` to `Result` and then
            // back, because we can `collect` into a `Result`, but not an
            // `Option`.
            indices
                .iter()
                .map(|index| types.inner.get(index).cloned().ok_or(()))
                .collect::<Result<Vec<_>, _>>()
                .ok()
        });
        let [inputs, outputs] = [inputs?, outputs?];

        Some(ConcreteSignature { inputs, outputs })
    }
}

/// # A concrete signature
///
/// Most code should use `Signature` instead, which references into signatures
/// stored in `Types`.
///
/// This type is only intended for type signatures that have a lifetime
/// extending beyond that of a running compiler. Like those of intrinsic or host
/// functions.
#[derive(Debug, Eq, PartialEq)]
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
