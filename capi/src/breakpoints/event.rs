use crate::runtime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    ClearEphemeral { location: runtime::Location },
}
