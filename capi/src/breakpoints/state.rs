use std::collections::BTreeSet;

use crate::runtime;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State {
    pub durable: BTreeSet<runtime::Location>,
    pub ephemeral: BTreeSet<runtime::Location>,
}
