use crate::runtime;

pub enum Event {
    MostRecentStep { location: runtime::Location },
    Finish,
}
