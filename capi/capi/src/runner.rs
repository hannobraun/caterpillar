use std::time::{Duration, Instant};

use capi_runtime::{Effect, Program, ProgramEffect, ProgramState, Value};

use crate::{display::TILES_PER_AXIS, server::EventsRx, updates::UpdatesTx};

pub struct Runner {
    inner: RunnerInner,
}

impl Runner {
    pub fn new(program: Program, events: EventsRx, updates: UpdatesTx) -> Self {
        Self {
            inner: RunnerInner {
                program,
                events,
                updates,
            },
        }
    }

    pub fn run(&mut self, mut handler: impl FnMut(DisplayEffect)) {
        let start_of_execution = Instant::now();
        let timeout = Duration::from_millis(10);

        loop {
            while let Ok(event) = self.inner.events.try_recv() {
                // This doesn't work so well. This receive loop was moved here,
                // so we can have some control over the program from the
                // debugger, while it is stuck in an endless loop.
                //
                // And this works somewhat. We can send events. But unless those
                // events result in the program to stop running, we won't see
                // any indication of them being received in the debugger, as the
                // program isn't sent when it's running.

                self.inner.program.apply_debug_event(event);
                self.inner.updates.send(&self.inner.program);
            }

            // This block needs to be located here, as receiving events from the
            // client can lead to a reset, which then must result in the
            // arguments being available, or the program can't work correctly.
            if let ProgramState::Finished = self.inner.program.state {
                self.inner.program.reset();
                self.inner
                    .program
                    .push([Value(TILES_PER_AXIS.try_into().unwrap()); 2]);
            }

            self.inner.program.step();
            match &self.inner.program.state {
                ProgramState::Running => {}
                ProgramState::Paused { .. } => {
                    break;
                }
                ProgramState::Finished => {
                    assert_eq!(
                        self.inner.program.evaluator.data_stack.num_values(),
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

                            self.inner.program.state = ProgramState::Running;
                        }
                    },
                },
            }

            if start_of_execution.elapsed() > timeout {
                self.inner.program.halt();
            }
        }

        self.inner.updates.send(&self.inner.program);
    }
}

struct RunnerInner {
    program: Program,
    events: EventsRx,
    updates: UpdatesTx,
}

pub enum DisplayEffect {
    SetTile { x: u8, y: u8, value: u8 },
}
