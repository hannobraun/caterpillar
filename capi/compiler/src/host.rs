pub trait Host {
    fn effect_number_to_function_name(effect: u8) -> Option<&'static str>;
    fn function_name_to_effect_number(name: &str) -> Option<u8>;
}

pub struct NoHost {}

impl Host for NoHost {
    fn effect_number_to_function_name(_effect: u8) -> Option<&'static str> {
        None
    }

    fn function_name_to_effect_number(_name: &str) -> Option<u8> {
        None
    }
}
