use std::iter;

use tokio::sync::mpsc;

use crate::{
    debugger::DebugEvent,
    effects::{DisplayEffect, EffectsRx, EffectsTx},
    program::Program,
    updates::UpdatesTx,
};

pub fn runner(
    program: Program,
    updates: UpdatesTx,
) -> (EventsTx, RunnerHandle, Runner) {
    let (events_tx, events_rx) = mpsc::unbounded_channel();
    let (effects_tx, effects_rx) = mpsc::unbounded_channel();

    let runner = Runner {
        program,
        events: events_rx,
        updates,
        effects_tx: EffectsTx { inner: effects_tx },
    };

    let handle = RunnerHandle { effects_rx };

    (events_tx, handle, runner)
}

pub type EventsRx = mpsc::UnboundedReceiver<DebugEvent>;
pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;

pub struct RunnerHandle {
    effects_rx: EffectsRx,
}

impl RunnerHandle {
    pub fn effects(&mut self) -> impl Iterator<Item = DisplayEffect> + '_ {
        iter::from_fn(|| self.effects_rx.try_recv().ok())
    }
}

pub struct Runner {
    pub program: Program,
    pub events: EventsRx,
    pub updates: UpdatesTx,
    pub effects_tx: EffectsTx,
}

impl Runner {
    pub async fn step(&mut self) -> Option<DisplayEffect> {
        self.updates.send_if_relevant_change(&self.program);

        None
    }
}
