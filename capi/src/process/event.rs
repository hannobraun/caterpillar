use crate::runtime::EvaluatorEffect;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    EffectTriggered { effect: EvaluatorEffect },
    EffectHandled,
    Finished,
}
