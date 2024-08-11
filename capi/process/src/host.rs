use std::fmt::Debug;

pub trait Host {
    fn function_name_to_effect_number(name: &str) -> Option<u8>;
}

pub trait HostEffect:
    Clone + Debug + Eq + for<'de> serde::Deserialize<'de> + serde::Serialize
{
    fn to_number(self) -> u8;
}

impl HostEffect for () {
    fn to_number(self) -> u8 {
        0
    }
}

pub struct NoHost {}

impl Host for NoHost {
    fn function_name_to_effect_number(_name: &str) -> Option<u8> {
        None
    }
}
