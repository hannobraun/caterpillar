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
    pub effects_rx: EffectsRx,
}

pub struct Runner {
    pub program: Program,
    pub events: EventsRx,
    pub updates: UpdatesTx,
    pub effects_tx: EffectsTx,
}
