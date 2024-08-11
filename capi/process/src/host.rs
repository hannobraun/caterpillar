use std::fmt::Debug;

pub trait Host {
    type Effect: HostEffect;

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
    type Effect = ();

    fn function_name_to_effect_number(_name: &str) -> Option<u8> {
        None
    }
}
