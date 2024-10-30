pub trait Host {
    fn effect_number_to_function_name(
        &self,
        effect: u8,
    ) -> Option<&'static str>;
    fn function_name_to_effect_number(name: &str) -> Option<u8>;
}

/// # A function that is provided by the host
pub trait HostFunction {
    /// # The name of the function
    fn name(&self) -> &'static str;
}

pub struct NoHost {}

impl Host for NoHost {
    fn effect_number_to_function_name(&self, _: u8) -> Option<&'static str> {
        None
    }

    fn function_name_to_effect_number(_: &str) -> Option<u8> {
        None
    }
}
