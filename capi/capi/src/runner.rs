use std::{sync::mpsc, thread};

use capi_runtime::{
    debugger::DebugEvent, BuiltinEffect, EvaluatorEffect, Program,
    ProgramEffect, ProgramEffectKind, Value,
};

use crate::{
    display::TILES_PER_AXIS,
    effects::{DisplayEffect, EffectsRx, EffectsTx, ResumeRx, ResumeTx},
    server::EventsRx,
    updates::UpdatesTx,
};

pub struct RunnerThread {
    effects: EffectsRx,
    resume: ResumeTx,
}

impl RunnerThread {
    pub fn start(
        program: Program,
        events: EventsRx,
        updates: UpdatesTx,
    ) -> Self {
        let (effects_tx, effects_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();

        let mut runner = Runner {
            program,
            events,
            updates,
            effects: EffectsTx { inner: effects_tx },
            resume: resume_rx,
        };

        thread::spawn(move || {
            runner.start();
        });

        Self {
            effects: effects_rx,
            resume: resume_tx,
        }
    }

    pub fn effects(&mut self) -> impl Iterator<Item = DisplayEffect> + '_ {
        self.effects.try_iter()
    }

    pub fn resume(&mut self) {
        // This will cause an error, if the other end has hung up, which happens
        // if the program has ended. Nothing we can do about it.
        let _ = self.resume.send(());
    }
}

struct Runner {
    program: Program,
    events: EventsRx,
    updates: UpdatesTx,
    effects: EffectsTx,
    resume: ResumeRx,
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
                // This doesn't work so well. This receive loop was moved here,
                // so we can have some control over the program from the
                // debugger, while it is stuck in an endless loop.
                //
                // And this works somewhat. We can send events. But unless those
                // events result in the program to stop running, we won't see
                // any indication of them being received in the debugger, as the
                // program isn't sent when it's running.

                match event {
                    DebugEvent::Continue => {
                        if let Some(ProgramEffect {
                            kind: ProgramEffectKind::Paused,
                            ..
                        }) = self.program.effects.front()
                        {
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
                                self.program.evaluator.next_instruction,
                            );
                            self.program.effects.pop_front();
                        } else {
                            println!(
                                "Debugger tried to step, but the program isn't \
                                paused."
                            );
                        }
                    }
                    DebugEvent::ToggleBreakpoint { address } => {
                        self.program.breakpoints.toggle_durable_at(address);
                    }
                }
            }

            self.program.step();
            if let Some(ProgramEffect {
                kind:
                    ProgramEffectKind::Evaluator(EvaluatorEffect::Builtin(effect)),
                ..
            }) = self.program.effects.front()
            {
                match effect {
                    BuiltinEffect::Load { address } => {
                        let address: usize = (*address).into();
                        let value = self.program.memory.inner[address];
                        self.program.push([Value(value)]);

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
                        self.program.effects.pop_front();

                        self.effects.send(DisplayEffect::SubmitTiles);

                        // This effect serves as a synchronization point between
                        // the program and the display code. Before we continue
                        // running, we need to wait here, until the display code
                        // has confirmed that we're ready to continue.
                        self.resume.recv().unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
}

// I don't like the `as` here, but `.try_into().unwrap()` doesn't work in a
// const context.
const ARGUMENTS: [Value; 2] = [Value(TILES_PER_AXIS as u8); 2];
