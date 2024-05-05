use std::time::{Duration, Instant};

use capi_runtime::{Effect, Program, ProgramEffect, ProgramState, Value};

use crate::{
    display::{TILES_OFFSET_IN_MEMORY, TILES_PER_AXIS},
    server::EventsRx,
    updates::UpdatesTx,
};

pub struct Runner {
    program: Program,
    events: EventsRx,
    updates: UpdatesTx,
}

impl Runner {
    pub fn new(program: Program, events: EventsRx, updates: UpdatesTx) -> Self {
        Self {
            program,
            events,
            updates,
        }
    }

    pub fn run(&mut self, _handler: impl FnMut(DisplayEffect), mem: &mut [u8]) {
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
                            let x_usize: usize = (*x).into();
                            let y_usize: usize = (*y).into();

                            let index = || {
                                x_usize
                                    .checked_add(
                                        y_usize.checked_mul(TILES_PER_AXIS)?,
                                    )?
                                    .checked_add(TILES_OFFSET_IN_MEMORY)
                            };
                            let index = index().unwrap();

                            mem[index] = *value;

                            self.program.state = ProgramState::Running;
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

pub enum DisplayEffect {}
