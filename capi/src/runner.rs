use tokio::sync::mpsc;

use crate::{
    debugger::DebugEvent,
    effects::{EffectsRx, EffectsTx},
    program::Program,
    updates::UpdatesTx,
};

pub fn runner(program: Program, updates: UpdatesTx) -> (EffectsRx, Runner) {
    let (effects_tx, effects_rx) = mpsc::unbounded_channel();

    let runner = Runner {
        program,
        updates,
        effects_tx: EffectsTx { inner: effects_tx },
    };

    (effects_rx, runner)
}

pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;

pub struct Runner {
    pub program: Program,
    pub updates: UpdatesTx,
    pub effects_tx: EffectsTx,
}
