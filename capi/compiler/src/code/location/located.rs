use std::ops::{Deref, DerefMut};

/// # A fragment of code, with its location attached
#[derive(Clone, Copy, Debug)]
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
