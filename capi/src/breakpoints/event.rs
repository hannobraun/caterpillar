use crate::runtime;

pub enum Event {
    SetDurable { location: runtime::Location },
    ClearDurable { location: runtime::Location },
}
