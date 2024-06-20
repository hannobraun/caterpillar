use crate::runtime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    SetEphemeral { location: runtime::Location },
    ClearEphemeral { location: runtime::Location },
}
