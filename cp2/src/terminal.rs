use std::{
    panic::{self, AssertUnwindSafe},
    time::Duration,
};

use crossterm::{
    event::{
        Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
    },
    terminal,
};
use futures::{FutureExt, StreamExt};
use tokio::time;

pub async fn run(frame_time: Duration, f: impl FnMut()) -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    let result = AssertUnwindSafe(run_inner(frame_time, f))
        .catch_unwind()
        .await;
    terminal::disable_raw_mode()?;

    match result {
        Ok(result) => result,
        Err(err) => panic::resume_unwind(err),
    }
}

async fn run_inner(
    frame_time: Duration,
    mut f: impl FnMut(),
) -> anyhow::Result<()> {
    let mut events = EventStream::new();

    let mut interval = time::interval(frame_time);
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        let event = tokio::select! {
            _ = interval.tick() => {
                None
            }
            event = events.next() => {
                Some(event)
            }
        };

        if let Some(event) = event {
            let Some(event) = event else {
                    anyhow::bail!("Error reading input event");
                };
            let event = event?;

            if let Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) = event
            {
                if modifiers.contains(KeyModifiers::CONTROL) {
                    // CTRL-C
                    break;
                }
            }
        }

        f()
    }

    Ok(())
}
