use std::{sync::mpsc, thread};

use capi_runtime::{
    debugger::DebugEvent,
    runtime::{BuiltinEffect, EvaluatorEffectKind, Value},
    Program, ProgramEffect, ProgramEffectKind,
};
use rand::random;

use crate::{
    display::TILES_PER_AXIS,
    effects::{DisplayEffect, EffectsRx, EffectsTx},
    server::EventsRx,
    updates::UpdatesTx,
};

pub struct RunnerThread {
    effects: EffectsRx,
}

impl RunnerThread {
    pub fn start(
        program: Program,
        events: EventsRx,
        updates: UpdatesTx,
    ) -> Self {
        let (effects_tx, effects_rx) = mpsc::channel();

        let mut runner = Runner {
            program,
            events,
            updates,
            effects: EffectsTx { inner: effects_tx },
        };

        thread::spawn(move || {
            runner.start();
        });

        Self {
            effects: effects_rx,
        }
    }

    pub fn effects(&mut self) -> impl Iterator<Item = DisplayEffect> + '_ {
        self.effects.try_iter()
    }
}

struct Runner {
    program: Program,
    events: EventsRx,
    updates: UpdatesTx,
    effects: EffectsTx,
}

impl Runner {
    fn start(&mut self) {
        self.program.push(ARGUMENTS);

        loop {
            self.updates.send_if_relevant_change(&self.program);

            let mut event = if self.program.can_step() {
                None
            } else {
                // If we're not running, the program won't step anyway, and
                // there's no point in busy-looping while nothing changes.
                //
                // Just wait until we receive an event from the client.
                Some(self.events.blocking_recv().unwrap())
            };

            // We either already have an event available here, if the program
            // wasn't running and we waited for one, or we might not. Either way
            // process the event that might or might not be available, as well
            // as all other events we can get our hands on.
            while let Some(event) =
                event.take().or_else(|| self.events.try_recv().ok())
            {
                match event {
                    DebugEvent::Continue { and_stop_at } => {
                        if let Some(ProgramEffect {
                            kind: ProgramEffectKind::Paused,
                            ..
                        }) = self.program.effects.front()
                        {
                            if let Some(instruction) = and_stop_at {
                                self.program
                                    .breakpoints
                                    .set_ephemeral(instruction);
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
                    ProgramEffectKind::Evaluator(EvaluatorEffectKind::Builtin(
                        effect,
                    )),
                ..
            }) = self.program.effects.front()
            {
                match effect {
                    BuiltinEffect::Error(_) => {
                        // Nothing needs to be done. With an unhandled effect,
                        // the program won't continue running, and the debugger
                        // will see the error and display it.
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

                        self.effects.send(DisplayEffect::SetTile {
                            x,
                            y,
                            value,
                        });

                        self.program.effects.pop_front();
                    }
                    BuiltinEffect::SubmitFrame => {
                        // This effect serves as a synchronization point between
                        // the program and the display code. Before we continue
                        // running, we need to wait here, until the display code
                        // has confirmed that we're ready to continue.
                        let (tx, rx) = mpsc::channel();
                        self.effects
                            .send(DisplayEffect::SubmitTiles { reply: tx });
                        let () = rx.recv().unwrap();

                        self.program.effects.pop_front();
                    }
                    BuiltinEffect::ReadInput => {
                        let (tx, rx) = mpsc::channel();

                        self.effects
                            .send(DisplayEffect::ReadInput { reply: tx });
                        let input = rx.recv().unwrap();

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
}

// I don't like the `as` here, but `.try_into().unwrap()` doesn't work in a
// const context.
const ARGUMENTS: [Value; 2] = [Value(TILES_PER_AXIS as i8); 2];
