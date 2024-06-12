use tokio::sync::mpsc;

use crate::{
    debugger::DebugEvent,
    effects::{EffectsRx, EffectsTx},
    program::Program,
    updates::UpdatesTx,
};

pub fn runner(
    program: Program,
    updates: UpdatesTx,
) -> (EventsTx, EffectsRx, Runner) {
    let (events_tx, events_rx) = mpsc::unbounded_channel();
    let (effects_tx, effects_rx) = mpsc::unbounded_channel();

    let runner = Runner {
        program,
        events: events_rx,
        updates,
        effects_tx: EffectsTx { inner: effects_tx },
    };

    (events_tx, effects_rx, runner)
}

pub type EventsRx = mpsc::UnboundedReceiver<DebugEvent>;
pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;

pub struct Runner {
    pub program: Program,
    pub events: EventsRx,
    pub updates: UpdatesTx,
    pub effects_tx: EffectsTx,
}
