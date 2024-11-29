use crate::code::{IndexMap, Located, NamedFunction};

/// # The syntax tree
///
/// See [parent module](super).
#[derive(Default)]
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
}
