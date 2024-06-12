use std::collections::VecDeque;

use tokio::sync::mpsc;

use crate::{
    debugger::DebugEvent,
    display::Display,
    ffi,
    games::{self, snake::snake},
    program::Program,
    tiles::NUM_TILES,
    ui,
    updates::{updates, UpdatesTx},
};

pub struct RuntimeState {
    pub program: Program,
    pub input: Input,
    pub on_frame: mpsc::UnboundedSender<()>,
    pub tiles: [u8; NUM_TILES],
    pub display: Option<Display>,
    pub events_rx: mpsc::UnboundedReceiver<DebugEvent>,
    pub updates_tx: UpdatesTx,
}

impl Default for RuntimeState {
    fn default() -> Self {
        let program = games::build(snake);

        let input = Input::default();
        let (on_frame_tx, _) = mpsc::unbounded_channel();
        let (updates_tx, updates_rx) = updates(&program);
        let (events_tx, events_rx) = mpsc::unbounded_channel();

        ui::start(updates_rx, events_tx);

        // While we're still using `pixels`, the `Display` constructor needs to
        // be async. We need to do some acrobatics here to deal with that.
        leptos::spawn_local(async {
            let display = Display::new().await.unwrap();

            let mut state = ffi::STATE.inner.lock().unwrap();
            let state = state.get_or_insert_with(Default::default);

            state.display = Some(display);
        });

        Self {
            program,
            input,
            on_frame: on_frame_tx,
            tiles: [0; NUM_TILES],
            display: None,
            events_rx,
            updates_tx,
        }
    }
}

#[derive(Default)]
pub struct Input {
    pub buffer: VecDeque<u8>,
}
