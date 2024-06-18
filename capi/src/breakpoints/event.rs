use crate::runtime;

pub enum Event {
    SetDurable { location: runtime::Location },
    ClearDurable { location: runtime::Location },
    SetEphemeral { location: runtime::Location },
    ClearEphemeral { location: runtime::Location },
}
