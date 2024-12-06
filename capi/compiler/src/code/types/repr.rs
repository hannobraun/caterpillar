use std::collections::BTreeMap;

use crate::code::syntax::{MemberLocation, SyntaxTree};

use super::resolve::resolve_type_annotations;

/// # The types that are explicitly specified in the code
///
/// ## Implementation Note
///
/// The long-term goal for Caterpillar is to be fully inferred, with no explicit
/// types annotations being necessary at all. Then this type can be removed.
#[derive(Debug)]
pub struct ExplicitTypes {
    inner: BTreeMap<MemberLocation, Signature>,
}

impl ExplicitTypes {
    /// # Resolve all explicit type annotations
    pub fn resolve(syntax_tree: &SyntaxTree) -> Self {
        let types_ = resolve_type_annotations(syntax_tree);
        Self { inner: types_ }
    }

    /// # Access the signature of the expression at the given location, if any
    pub fn signature_of(
        &self,
        location: &MemberLocation,
    ) -> Option<&Signature> {
        self.inner.get(location)
    }
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
}

/// # A type signature that applies to a function or expression
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
    /// # The inputs that the function or expression consumes
    pub inputs: Vec<Type>,

    /// # The outputs that the function or expression produces
    pub outputs: Vec<Type>,
}

impl<I, O> From<(I, O)> for Signature
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
