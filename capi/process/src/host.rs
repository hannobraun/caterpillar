use std::fmt::Debug;

use crate::HostEffect;

pub trait Host: Clone + Debug + Eq {
    type Effect;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefaultHost;

impl Host for DefaultHost {
    type Effect = HostEffect;
}
