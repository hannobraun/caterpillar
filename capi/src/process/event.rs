use crate::runtime::{self, EvaluatorEffect};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    HasStepped { location: runtime::Location },
    TriggerEffect { effect: EvaluatorEffect },
    HandleEffect,
    Finish,
}
