use crate::runtime::{self, EvaluatorEffect};

pub enum Event {
    Step { location: runtime::Location },
    TriggerEffect { effect: EvaluatorEffect },
    Finish,
}
