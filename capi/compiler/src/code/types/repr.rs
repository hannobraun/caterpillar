use std::collections::BTreeMap;

use crate::code::{
    syntax::{MemberLocation, SyntaxTree},
    FunctionCalls,
};

use super::{infer::infer_expression, resolve::resolve_type_annotations};

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

/// # The resolved types
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Types {
    inner: BTreeMap<MemberLocation, Signature>,
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
        let mut types_ = BTreeMap::new();

        for function in syntax_tree.all_functions() {
            for branch in function.branches() {
                for expression in branch.expressions() {
                    let inferred =
                        infer_expression(expression.fragment, function_calls);
                    let explicit =
                        explicit_types.signature_of(&expression.location);

                    if let (Some(inferred), Some(explicit)) =
                        (inferred.as_ref(), explicit)
                    {
                        panic!(
                            "Type that could be inferred was also specified \
                            explicitly. This is currently not allowed, as the \
                            goal is to transition away from explicit \
                            annotations completely.\n\
                            \n\
                            Inferred type: {inferred:?}\n\
                            Explicit type: {explicit:?}\n\
                            \n\
                            At {}\n",
                            expression.location.display(syntax_tree),
                        );
                    }

                    if let Some(signature) = inferred.or(explicit.cloned()) {
                        types_.insert(expression.location, signature);
                    }
                }
            }
        }

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
