use crate::code::{IndexMap, Located, NamedFunction};

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
        self.named_functions.iter().find_map(|(&index, function)| {
            if function.name == name {
                Some(Located {
                    fragment: function,
                    location: index,
                })
            } else {
                None
            }
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
}
