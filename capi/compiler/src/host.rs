use crate::code::ConcreteSignature;

/// # A host into which a Caterpillar application is embedded
pub trait Host {
    /// # Iterate over all of the host's functions
    ///
    /// Implementations must guarantee that each function has a unique number
    /// (see [`HostFunction::number`]) and name (see [`HostFunction::name`]).
    fn functions(&self) -> impl IntoIterator<Item = &dyn HostFunction>;

    /// # Access a host function by its number
    ///
    /// Return `None`, if the provided number does not identify a host function.
    fn function_by_number(&self, number: &u8) -> Option<&dyn HostFunction>;

    /// # Access a host function by its name
    ///
    /// Returns `None`, if the provided name does not identify a host function.
    fn function_by_name(&self, name: &str) -> Option<&dyn HostFunction>;
}

/// # A function that is provided by the host
pub trait HostFunction {
    /// # The number that identifies the function in the host effect
    fn number(&self) -> u8;

    /// # The name that identifies the function in input code
    fn name(&self) -> &'static str;

    /// # The type signature of the function
    fn signature(&self) -> ConcreteSignature;
}

pub struct NoHost;

impl Host for NoHost {
    fn functions(&self) -> impl IntoIterator<Item = &dyn HostFunction> {
        None
    }

    fn function_by_number(&self, _: &u8) -> Option<&dyn HostFunction> {
        None
    }

    fn function_by_name(&self, _: &str) -> Option<&dyn HostFunction> {
        None
    }
}
