use std::iter;

use crate::code::{Function, IndexMap, Located, NamedFunction};

/// # The syntax tree
///
/// See [parent module](super).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SyntaxTree {
    pub named_functions: IndexMap<NamedFunction>,
}

impl SyntaxTree {
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
