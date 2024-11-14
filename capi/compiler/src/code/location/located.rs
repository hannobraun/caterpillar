use std::ops::{Deref, DerefMut};

use crate::code::{Function, Index, NamedFunction};

use super::FunctionLocation;

/// # A fragment of code, with its location attached
#[derive(Clone, Debug)]
pub struct Located<T: HasLocation> {
    /// # The code fragment
    pub fragment: T,

    /// # The location of the code fragment
    pub location: T::Location,
}

impl<T: HasLocation> Deref for Located<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.fragment
    }
}

impl<T: HasLocation> DerefMut for Located<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fragment
    }
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

/// # Implemented by all code fragments, to abstract over their location
pub trait HasLocation {
    /// # The location of this fragment
    type Location;
}

impl<T> HasLocation for &T
where
    T: HasLocation,
{
    type Location = T::Location;
}

impl<T> HasLocation for &mut T
where
    T: HasLocation,
{
    type Location = T::Location;
}
