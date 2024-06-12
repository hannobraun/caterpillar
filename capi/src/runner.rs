use std::iter;

use rand::random;
use tokio::sync::mpsc;

use crate::{
    debugger::DebugEvent,
    effects::{DisplayEffect, EffectsRx, EffectsTx},
    program::{Program, ProgramEffect, ProgramEffectKind},
    runtime::{BuiltinEffect, EvaluatorEffectKind, Value},
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
    pub async fn step(
        &mut self,
        events: Vec<DebugEvent>,
    ) -> Option<DisplayEffect> {
        for event in events.into_iter() {
            match event {
                DebugEvent::Continue { and_stop_at } => {
                    if let Some(ProgramEffect {
                        kind: ProgramEffectKind::Paused,
                        ..
                    }) = self.program.effects.front()
                    {
                        if let Some(instruction) = and_stop_at {
                            self.program.breakpoints.set_ephemeral(instruction);
                        }

                        self.program.effects.pop_front();
                    }
                }
                DebugEvent::Reset => {
                    self.program.reset();
                }
                DebugEvent::Step => {
                    if let Some(ProgramEffect {
                        kind: ProgramEffectKind::Paused,
                        ..
                    }) = self.program.effects.front()
                    {
                        self.program.breakpoints.set_ephemeral(
                            self.program.evaluator.next_instruction(),
                        );
                        self.program.effects.pop_front();
                    }
                }
                DebugEvent::Stop => {
                    self.program.breakpoints.set_ephemeral(
                        self.program.evaluator.next_instruction(),
                    );
                }
                DebugEvent::ToggleBreakpoint { location } => {
                    self.program.breakpoints.toggle_durable_at(location);
                }
            }
        }

        self.program.step();
        if let Some(ProgramEffect {
            kind:
                ProgramEffectKind::Evaluator(EvaluatorEffectKind::Builtin(effect)),
            ..
        }) = self.program.effects.front()
        {
            match effect {
                BuiltinEffect::Error(_) => {
                    // Nothing needs to be done. With an unhandled effect, the
                    // program won't continue running, and the debugger will see
                    // the error and display it.
                }
                BuiltinEffect::Load { address } => {
                    let address: usize = (*address).into();
                    let value = self.program.memory.inner[address];
                    self.program.push([value]);

                    self.program.effects.pop_front();
                }
                BuiltinEffect::Store { address, value } => {
                    let address: usize = (*address).into();
                    self.program.memory.inner[address] = *value;

                    self.program.effects.pop_front();
                }
                BuiltinEffect::SetTile { x, y, value } => {
                    let x = *x;
                    let y = *y;
                    let value = *value;

                    self.effects_tx.send(DisplayEffect::SetTile {
                        x,
                        y,
                        value,
                    });

                    self.program.effects.pop_front();
                }
                BuiltinEffect::SubmitFrame => {
                    // This effect serves as a synchronization point between the
                    // program and the display code. Before we continue running,
                    // we need to wait here, until the display code has
                    // confirmed that we're ready to continue.
                    let (tx, mut rx) = mpsc::unbounded_channel();
                    self.effects_tx
                        .send(DisplayEffect::SubmitTiles { reply: tx });
                    let () = rx.recv().await.unwrap();

                    self.program.effects.pop_front();
                }
                BuiltinEffect::ReadInput => {
                    let (tx, mut rx) = mpsc::unbounded_channel();

                    self.effects_tx
                        .send(DisplayEffect::ReadInput { reply: tx });
                    let input = rx.recv().await.unwrap();

                    self.program.push([Value(input)]);
                    self.program.effects.pop_front();
                }
                BuiltinEffect::ReadRandom => {
                    self.program.push([Value(random())]);
                    self.program.effects.pop_front();
                }
            }
        }

        self.updates.send_if_relevant_change(&self.program);

        None
    }
}
