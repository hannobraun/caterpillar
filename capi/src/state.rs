use std::collections::VecDeque;

use tokio::sync::mpsc::error::TryRecvError;

use crate::{
    display::Display,
    ffi,
    games::{self, snake::snake},
    runner::{runner, RunnerHandle},
    tiles::NUM_TILES,
    ui::{self, handle_updates},
    updates::updates,
};

pub struct RuntimeState {
    pub input: Input,
    pub runner: Runner,
    pub tiles: [u8; NUM_TILES],
    pub display: Option<Display>,
}

impl Default for RuntimeState {
    fn default() -> Self {
        let program = games::build(snake);

        let input = Input::default();
        let (updates_tx, updates_rx) = updates(&program);
        let (events_tx, runner_handle, mut runner) =
            runner(program, updates_tx);

        leptos::spawn_local(async move {
            loop {
                loop {
                    match runner.events.try_recv() {
                        Ok(event) => {
                            runner.program.process_event(event);
                        }
                        Err(TryRecvError::Empty) => {
                            break;
                        }
                        Err(TryRecvError::Disconnected) => {
                            // The other end has hung up, which happens during
                            // shutdown. Shut down this task, too.
                            return;
                        }
                    }
                }

                if !runner.program.can_step() {
                    // If the program won't step anyway, then there's no point
                    // in busy-looping while nothing changes.
                    //
                    // Just wait until we receive an event from the client.
                    let event = runner.events.recv().await.unwrap();
                    runner.program.process_event(event);
                }

                runner.program.step();

                runner.step().await;
            }
        });

        let set_program = ui::start(events_tx.clone());
        leptos::spawn_local(handle_updates(updates_rx, set_program));

        // While we're still using `pixels`, the `Display` constructor needs to
        // be async. We need to do some acrobatics here to deal with that.
        leptos::spawn_local(async {
            let display = Display::new().await.unwrap();

            let mut state = ffi::STATE.inner.lock().unwrap();
            let state = state.get_or_insert_with(Default::default);

            state.display = Some(display);
        });

        Self {
            input,
            runner: Runner {
                handle: runner_handle,
            },
            tiles: [0; NUM_TILES],
            display: None,
        }
    }
}

#[derive(Default)]
pub struct Input {
    pub buffer: VecDeque<u8>,
}

pub struct Runner {
    pub handle: RunnerHandle,
}
