use std::{
    future::Future,
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
use tokio::time::{self, Interval};

pub async fn run<F, R>(frame_time: Duration, f: F) -> anyhow::Result<()>
where
    F: FnMut() -> R,
    R: Future<Output = anyhow::Result<()>>,
{
    let events = EventStream::new();

    let mut interval = time::interval(frame_time);
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    terminal::enable_raw_mode()?;
    let result = AssertUnwindSafe(run_inner(events, interval, f))
        .catch_unwind()
        .await;
    terminal::disable_raw_mode()?;

    match result {
        Ok(result) => result,
        Err(err) => panic::resume_unwind(err),
    }
}

async fn run_inner<F, R>(
    mut events: EventStream,
    mut interval: Interval,
    mut f: F,
) -> anyhow::Result<()>
where
    F: FnMut() -> R,
    R: Future<Output = anyhow::Result<()>>,
{
    f().await?;

    loop {
        let () = match next_event(&mut events, &mut interval).await? {
            Some(()) => (),
            None => break,
        };

        f().await?;
    }

    Ok(())
}

pub async fn next_event(
    events: &mut EventStream,
    _: &mut Interval,
) -> anyhow::Result<Option<()>> {
    let event = tokio::select! {
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
                return Ok(None);
            }
        }
    }

    Ok(Some(()))
}

pub struct Size {
    pub num_columns: usize,
    pub num_rows: usize,
}

impl Size {
    pub fn get() -> anyhow::Result<Self> {
        let (num_columns, num_rows) = terminal::size()?;
        let (num_columns, num_rows) = (num_columns as usize, num_rows as usize);

        Ok(Size {
            num_columns,
            num_rows,
        })
    }
}
