use crate::runtime;

pub enum Event {
    SetDurable { location: runtime::Location },
}
