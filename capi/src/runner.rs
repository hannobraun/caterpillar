use std::iter;

use rand::random;
use tokio::sync::mpsc::{self, error::TryRecvError};

use crate::{
    debugger::DebugEvent,
    effects::{DisplayEffect, EffectsRx, EffectsTx},
    program::{Program, ProgramEffect, ProgramEffectKind},
    runtime::{BuiltinEffect, EvaluatorEffectKind, Value},
    tiles::TILES_PER_AXIS,
    updates::UpdatesTx,
};

pub fn runner(
    program: Program,
    updates: UpdatesTx,
) -> (EventsTx, RunnerHandle, Runner) {
    let (events_tx, events_rx) = mpsc::unbounded_channel();
    let (effects_tx, effects_rx) = mpsc::unbounded_channel();

    let mut runner = Runner {
        program,
        events: events_rx,
        updates,
        effects_tx: EffectsTx { inner: effects_tx },
    };
    runner.program.push(ARGUMENTS);

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
    program: Program,
    events: EventsRx,
    updates: UpdatesTx,
    effects_tx: EffectsTx,
}

impl Runner {
    pub async fn step(&mut self) {
        self.updates.send_if_relevant_change(&self.program);

        let mut events = Vec::new();
        loop {
            match self.events.try_recv() {
                Ok(event) => events.push(event),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    // The other end has hung up, which happens during shutdown.
                    // Nothing we can do about it, except wait until this task
                    // is shut down too.
                    return;
                }
            }
        }

        if events.is_empty() && !self.program.can_step() {
            // If the program won't step anyway, then there's no point in
            // busy-looping while nothing changes.
            //
            // Just wait until we receive an event from the client.
            events.push(self.events.recv().await.unwrap());
        }

        // We either already have an event available here, if the program wasn't
        // running and we waited for one, or we might not. Either way process
        // the event that might or might not be available, as well as all other
        // events we can get our hands on.
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
                    } else {
                        println!(
                            "Debugger tried to continue, but the program \
                                isn't paused."
                        );
                    }
                }
                DebugEvent::Reset => {
                    self.program.reset();
                    self.program.push(ARGUMENTS);
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
                    } else {
                        println!(
                            "Debugger tried to step, but the program isn't \
                                paused."
                        );
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
    }
}

// I don't like the `as` here, but `.try_into().unwrap()` doesn't work in a
// const context.
const ARGUMENTS: [Value; 2] = [Value(TILES_PER_AXIS as i8); 2];
