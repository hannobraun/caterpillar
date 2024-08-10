use std::fmt::Debug;

pub trait Host {
    type Effect: Clone
        + Debug
        + Eq
        + for<'de> serde::Deserialize<'de>
        + serde::Serialize;

    fn function(name: &str) -> Option<Self::Effect>;
}

pub struct NoHost {}

impl Host for NoHost {
    type Effect = ();

    fn function(_name: &str) -> Option<Self::Effect> {
        None
    }
}
