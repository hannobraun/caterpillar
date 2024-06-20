use std::collections::VecDeque;

use crate::runtime::{self, EvaluatorEffect};

use super::Event;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProcessState {
    most_recent_step: Option<runtime::Location>,
    unhandled_effects: VecDeque<EvaluatorEffect>,
    has_finished: bool,
}

impl ProcessState {
    pub fn most_recent_step(&self) -> Option<runtime::Location> {
        self.most_recent_step.clone()
    }

    pub fn first_unhandled_effect(&self) -> Option<&EvaluatorEffect> {
        self.unhandled_effects.front()
    }

    pub fn is_running(&self) -> bool {
        !self.has_finished
    }

    pub fn has_finished(&self) -> bool {
        self.has_finished
    }

    pub fn can_step(&self) -> bool {
        self.is_running() && self.unhandled_effects.is_empty()
    }

    pub fn evolve(&mut self, event: Event) {
        match event {
            Event::HasStepped { location } => {
                self.most_recent_step = Some(location);
            }
            Event::EffectTriggered { effect } => {
                self.unhandled_effects.push_back(effect);
            }
            Event::EffectHandled => {
                self.unhandled_effects.pop_front();
            }
            Event::Finished => {
                self.has_finished = true;
            }
        }
    }
}
