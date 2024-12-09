use std::{collections::BTreeMap, fmt};

use crate::code::{
    syntax::{MemberLocation, SyntaxTree},
    FunctionCalls,
};

use super::{infer::infer_types, resolve::resolve_type_annotations};

/// # The types that are explicitly specified in the code
///
/// ## Implementation Note
///
/// The long-term goal for Caterpillar is to be fully inferred, with no explicit
/// types annotations being necessary at all. Then this type can be removed.
#[derive(Debug)]
pub struct ExplicitTypes {
    inner: Signatures,
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

/// # The resolved types
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Types {
    signatures: Signatures,
    stacks: Stacks,
}

impl Types {
    /// # Infer types
    ///
    /// ## Implementation Note
    ///
    /// This current also uses the explicitly specified types where it can't
    /// infer yet, but the plan is to remove any explicit annotations in favor
    /// of full type inference.
    pub fn infer(
        syntax_tree: &SyntaxTree,
        explicit_types: ExplicitTypes,
        function_calls: &FunctionCalls,
    ) -> Self {
        let (signatures, stacks) =
            infer_types(syntax_tree, &explicit_types, function_calls);
        Self { signatures, stacks }
    }

    /// # Access the signature of the expression at the given location, if any
    pub fn signature_of(
        &self,
        location: &MemberLocation,
    ) -> Option<&Signature> {
        self.signatures.get(location)
    }

    /// # Access the stack, as of the expression at the given location, if any
    ///
    /// The stack returned here is always local to the function that the
    /// expression is in. Tracking the global stack at compile-time is not
    /// possible, and only the local contents of the stack are relevant to what
    /// the expression can do anyway.
    pub fn stack_at(&self, location: &MemberLocation) -> Option<&[Type]> {
        self.stacks.get(location).map(|stack| &**stack)
    }
}

pub type Signatures = BTreeMap<MemberLocation, Signature>;
pub type Stacks = BTreeMap<MemberLocation, Stack>;
pub type Stack = Vec<Type>;

/// # A type signature that applies to a function or expression
///
/// This struct is generic over the type of the type in the signature. Usually
/// it's going to be [`Type`], but there are various specialized situation where
/// more specialized signatures are needed, and this type parameter enables that
/// without duplication.
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
pub struct Signature<T = Type> {
    /// # The inputs that the function or expression consumes
    pub inputs: Vec<T>,

    /// # The outputs that the function or expression produces
    pub outputs: Vec<T>,
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

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut inputs = self.inputs.iter().peekable();
        while let Some(input) = inputs.next() {
            write!(f, "{input}")?;

            if inputs.peek().is_some() {
                write!(f, ", ")?;
            }
        }

        write!(f, " ->")?;
        if !self.outputs.is_empty() {
            write!(f, " ")?;
        }

        let mut outputs = self.outputs.iter().peekable();
        while let Some(output) = outputs.next() {
            write!(f, "{output}")?;

            if outputs.peek().is_some() {
                write!(f, ", ")?;
            }
        }

        Ok(())
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Function { signature } => {
                write!(f, "fn {signature} end")?;
            }
            Self::Number => {
                write!(f, "Number")?;
            }
        }

        Ok(())
    }
}
