use crate::code::{Function, Index, NamedFunction};

use super::{located::HasLocation, FunctionLocation, Located};

impl HasLocation for NamedFunction {
    type Location = Index<NamedFunction>;
}

impl Located<&NamedFunction> {
    /// # Access the index of the found function
    ///
    /// This is a convenience accessor, to make code that would otherwise access
    /// `metadata` directly more readable.
    pub fn index(&self) -> Index<NamedFunction> {
        self.location
    }

    /// # Access the location of the found function
    pub fn location(&self) -> FunctionLocation {
        let index = self.location;
        index.into()
    }

    /// # Convert this located named function to a located function
    pub fn as_located_function(&self) -> Located<&Function> {
        Located {
            fragment: &self.fragment.inner,
            location: FunctionLocation::NamedFunction {
                index: self.location,
            },
        }
    }
}

impl Located<&mut NamedFunction> {
    /// # Convert this located named function to a located function
    pub fn as_located_function_mut(&mut self) -> Located<&mut Function> {
        Located {
            fragment: &mut self.fragment.inner,
            location: FunctionLocation::NamedFunction {
                index: self.location,
            },
        }
    }
}
