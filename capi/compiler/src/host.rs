pub trait Host {
    fn function_name_to_effect_number(name: &str) -> Option<u8>;
}

pub struct NoHost {}

impl Host for NoHost {
    fn function_name_to_effect_number(_name: &str) -> Option<u8> {
        None
    }
}
