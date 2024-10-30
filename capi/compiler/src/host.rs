pub trait Host {
    fn effect_number_to_function_name(
        &self,
        effect: u8,
    ) -> Option<&'static str>;

    fn function_by_name(&self, name: &str) -> Option<&dyn HostFunction>;
}

/// # A function that is provided by the host
pub trait HostFunction {
    /// # The number that identifies the function in the host effect
    fn number(&self) -> u8;

    /// # The name that identifies the function in input code
    fn name(&self) -> &'static str;
}

pub struct NoHost {}

impl Host for NoHost {
    fn effect_number_to_function_name(&self, _: u8) -> Option<&'static str> {
        None
    }

    fn function_by_name(&self, _: &str) -> Option<&dyn HostFunction> {
        None
    }
}
