mod event;
mod state;

pub use self::{event::Event, state::State};

use crate::runtime;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Breakpoints {
    state: State,
}

impl Breakpoints {
    pub fn set_durable(&mut self, location: runtime::Location) {
        self.state.evolve(Event::SetDurable { location })
    }

    pub fn clear_durable(&mut self, location: runtime::Location) {
        self.state.evolve(Event::ClearDurable { location })
    }

    pub fn set_ephemeral(&mut self, location: runtime::Location) {
        self.state.ephemeral.insert(location);
    }

    pub fn durable_breakpoint_at(&self, location: &runtime::Location) -> bool {
        self.state.durable_at(location)
    }

    pub fn should_stop_at_and_clear_ephemeral(
        &mut self,
        location: &runtime::Location,
    ) -> bool {
        let ephemeral = self.state.ephemeral.take(location).is_some();
        ephemeral || self.durable_breakpoint_at(location)
    }
}
