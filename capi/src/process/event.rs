use crate::runtime;

pub enum Event {
    Step { location: runtime::Location },
    Finish,
}
