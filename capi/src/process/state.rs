use std::collections::VecDeque;

use crate::runtime::{self, EvaluatorEffect};

use super::Event;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State {
    most_recent_step: Option<runtime::Location>,
    pub unhandled_effects: VecDeque<EvaluatorEffect>,
    has_finished: bool,
}

impl State {
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

    pub fn evolve(&mut self, event: Event) {
        match event {
            Event::Step { location } => {
                self.most_recent_step = Some(location);
            }
            Event::TriggerEffect { effect } => {
                self.unhandled_effects.push_back(effect);
            }
            Event::HandleEffect => {
                self.unhandled_effects.pop_front();
            }
            Event::Finish => {
                self.has_finished = true;
            }
        }
    }
}
