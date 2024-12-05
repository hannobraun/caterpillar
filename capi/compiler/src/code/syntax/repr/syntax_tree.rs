use std::iter;

use crate::code::{
    syntax::{parse::parse, FunctionLocation, Located},
    IndexMap, Tokens,
};

use super::function::{Function, NamedFunction};

/// # The syntax tree
///
/// See [parent module](super).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SyntaxTree {
    pub named_functions: IndexMap<NamedFunction>,
}

impl SyntaxTree {
    pub fn parse(tokens: Tokens) -> Self {
        let named_functions = parse(tokens);
        Self { named_functions }
    }

    /// # Find the function with the provided name
    ///
    /// Return `None`, if no function with this name can be found.
    pub fn function_by_name(
        &self,
        name: &str,
    ) -> Option<Located<&NamedFunction>> {
        self.named_functions()
            .find(|function| function.name == name)
    }

    /// # Find the top-level parent of a given function
    ///
    /// If the function at the provided location has no parent, the function
    /// itself is returned.
    ///
    /// Returns `None`, if there is no function at the provided location.
    pub fn find_top_level_parent_function(
        &self,
        location: &FunctionLocation,
    ) -> Option<Located<&NamedFunction>> {
        let index = match location {
            FunctionLocation::NamedFunction { index } => index,
            FunctionLocation::AnonymousFunction { location } => {
                return self
                    .find_top_level_parent_function(&location.parent.parent);
            }
        };

        let named_function = self.named_functions.get(index)?;

        Some(Located {
            fragment: named_function,
            location: *index,
        })
    }

    /// # Iterate over the named functions
    pub fn named_functions(
        &self,
    ) -> impl Iterator<Item = Located<&NamedFunction>> {
        self.named_functions
            .iter()
            .map(|(&index, function)| Located {
                fragment: function,
                location: index,
            })
    }

    /// # Iterate over all functions, both named and anonymous
    pub fn all_functions(&self) -> impl Iterator<Item = Located<&Function>> {
        self.named_functions().flat_map(|named_function| {
            let function = named_function.into_located_function();
            iter::once(function.clone()).chain(function.all_local_functions())
        })
    }
}
