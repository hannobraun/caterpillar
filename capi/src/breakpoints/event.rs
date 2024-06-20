use crate::runtime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    ClearDurable { location: runtime::Location },
    SetEphemeral { location: runtime::Location },
    ClearEphemeral { location: runtime::Location },
}
