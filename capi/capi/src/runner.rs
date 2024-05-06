use std::time::{Duration, Instant};

use capi_runtime::{Effect, Program, ProgramEffect, ProgramState, Value};

use crate::{display::TILES_PER_AXIS, server::EventsRx, updates::UpdatesTx};

pub struct RunnerThread {
    inner: Runner,
}

impl RunnerThread {
    pub fn new(program: Program, events: EventsRx, updates: UpdatesTx) -> Self {
        Self {
            inner: Runner {
                program,
                events,
                updates,
            },
        }
    }

    pub fn run(&mut self, handler: impl FnMut(DisplayEffect)) {
        self.inner.run(handler)
    }
}

struct Runner {
    program: Program,
    events: EventsRx,
    updates: UpdatesTx,
}

impl Runner {
    fn run(&mut self, mut handler: impl FnMut(DisplayEffect)) {
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
                            handler(DisplayEffect::SetTile { x, y, value });

                            self.program.state = ProgramState::Running;
                        }
                        Effect::RequestRedraw => {
                            self.program.state = ProgramState::Running;

                            // Nothing else to do here yet, until there's a
                            // matching `DisplayEffect` variant.
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

pub enum DisplayEffect {
    SetTile { x: u8, y: u8, value: u8 },
}
