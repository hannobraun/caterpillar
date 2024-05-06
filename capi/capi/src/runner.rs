use std::{
    sync::mpsc::{self, TryRecvError},
    time::{Duration, Instant},
};

use capi_runtime::{Effect, Program, ProgramEffect, ProgramState, Value};

use crate::{display::TILES_PER_AXIS, server::EventsRx, updates::UpdatesTx};

pub struct RunnerThread {
    inner: Runner,
    effects: EffectsRx,
    resume: ResumeTx,
}

impl RunnerThread {
    pub fn new(program: Program, events: EventsRx, updates: UpdatesTx) -> Self {
        let (effects_tx, effects_rx) = mpsc::channel();
        let (resume_tx, resume_rx) = mpsc::channel();

        Self {
            inner: Runner {
                program,
                events,
                updates,
                effects: effects_tx,
                resume: resume_rx,
            },
            effects: effects_rx,
            resume: resume_tx,
        }
    }

    pub fn start(&mut self) {
        self.inner.run()
    }

    pub fn effects(&mut self) -> impl Iterator<Item = DisplayEffect> + '_ {
        self.effects.try_iter()
    }

    pub fn resume(&mut self) {
        self.resume.send(()).unwrap();
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
    fn run(&mut self) {
        let start_of_execution = Instant::now();
        let timeout = Duration::from_millis(10);

        loop {
            while let Ok(event) = self.events.try_recv() {
                // This doesn't work so well. This receive loop was moved here,
                // so we can have some control over the program from the
                // debugger, while it is stuck in an endless loop.
                //
                // And this works somewhat. We can send events. But unless those
                // events result in the program to stop running, we won't see
                // any indication of them being received in the debugger, as the
                // program isn't sent when it's running.

                self.program.apply_debug_event(event);
                self.updates.send(&self.program);
            }

            // This block needs to be located here, as receiving events from the
            // client can lead to a reset, which then must result in the
            // arguments being available, or the program can't work correctly.
            if let ProgramState::Finished = self.program.state {
                self.program.reset();
                self.program
                    .push([Value(TILES_PER_AXIS.try_into().unwrap()); 2]);
            }

            self.program.step();
            match &self.program.state {
                ProgramState::Running => {}
                ProgramState::Paused { .. } => {
                    break;
                }
                ProgramState::Finished => {
                    assert_eq!(
                        self.program.evaluator.data_stack.num_values(),
                        0
                    );
                    break;
                }
                ProgramState::Effect { effect, .. } => match effect {
                    ProgramEffect::Halted => {
                        break;
                    }
                    ProgramEffect::Builtin(effect) => match effect {
                        Effect::Error(_) => {
                            break;
                        }
                        Effect::SetTile { x, y, value } => {
                            let x = *x;
                            let y = *y;
                            let value = *value;
                            self.effects
                                .send(DisplayEffect::SetTile { x, y, value })
                                .unwrap();

                            self.program.state = ProgramState::Running;
                        }
                        Effect::RequestRedraw => {
                            self.program.state = ProgramState::Running;

                            self.effects
                                .send(DisplayEffect::RequestRedraw)
                                .unwrap();

                            // The purpose of the "request redraw" effect is to
                            // serve as a synchronization point, where the
                            // program can pause until the display code has
                            // processed the effect (and all leading up to it).
                            //
                            // Once this is a dedicated thread, we need to wait
                            // here until a signal is received on the `resume`
                            // channel. But until then, we can't do that.
                            assert_eq!(
                                self.resume.try_recv(),
                                Err(TryRecvError::Empty)
                            );
                        }
                    },
                },
            }

            if start_of_execution.elapsed() > timeout {
                self.program.halt();
            }
        }

        self.updates.send(&self.program);
    }
}

type EffectsTx = mpsc::Sender<DisplayEffect>;
type EffectsRx = mpsc::Receiver<DisplayEffect>;

type ResumeTx = mpsc::Sender<()>;
type ResumeRx = mpsc::Receiver<()>;

#[derive(Debug)]
pub enum DisplayEffect {
    SetTile { x: u8, y: u8, value: u8 },
    RequestRedraw,
}
