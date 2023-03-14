use std::{
    future::Future,
    panic::{self, AssertUnwindSafe},
};

use crossterm::{
    event::{
        Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
    },
    terminal,
};
use futures::{FutureExt, StreamExt};

pub struct Terminal {
    events: EventStream,
}

impl Terminal {
    pub async fn run<F, R>(f: F) -> anyhow::Result<()>
    where
        F: FnMut() -> R,
        R: Future<Output = anyhow::Result<()>>,
    {
        let events = EventStream::new();

        terminal::enable_raw_mode()?;
        let result = AssertUnwindSafe(run_inner(Terminal { events }, f))
            .catch_unwind()
            .await;
        terminal::disable_raw_mode()?;

        match result {
            Ok(result) => result,
            Err(err) => panic::resume_unwind(err),
        }
    }

    pub async fn next_event(&mut self) -> anyhow::Result<Option<()>> {
        let event = self.events.next().await;

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

        Ok(Some(()))
    }
}

async fn run_inner<F, R>(mut terminal: Terminal, mut f: F) -> anyhow::Result<()>
where
    F: FnMut() -> R,
    R: Future<Output = anyhow::Result<()>>,
{
    f().await?;

    loop {
        let () = match terminal.next_event().await? {
            Some(()) => (),
            None => break,
        };

        f().await?;
    }

    Ok(())
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
