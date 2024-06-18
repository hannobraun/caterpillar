use crate::runtime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    SetDurable { location: runtime::Location },
    ClearDurable { location: runtime::Location },
    SetEphemeral { location: runtime::Location },
    ClearEphemeral { location: runtime::Location },
}
