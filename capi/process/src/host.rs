use std::fmt::Debug;

use crate::{Effect, Stack};

pub trait Host {
    type Effect: Clone
        + Debug
        + Eq
        + for<'de> serde::Deserialize<'de>
        + serde::Serialize;

    fn function(name: &str) -> Option<HostFunction<Self::Effect>>;
}

pub type HostFunction<H> = fn(&mut Stack) -> Result<(), Effect<H>>;

pub struct NoHost {}

impl Host for NoHost {
    type Effect = ();

    fn function(_name: &str) -> Option<HostFunction<Self::Effect>> {
        None
    }
}
