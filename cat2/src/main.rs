mod cells;
mod cp;
mod event_loop;
mod ui;

use std::{io, time::Duration};

use crossterm::{
    event::{
        Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
    },
    terminal,
};
use futures::StreamExt;
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;

    let mut events = EventStream::new();

    let mut state = event_loop::State {
        functions: cp::Functions::new(),
        generations: Vec::new(),
        buffer: ui::Buffer::new(),
        stdout: io::stdout(),
    };

    let delay = Duration::from_millis(125);
    let mut interval = time::interval(delay);
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

        let event = match event {
            Some(event) => {
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

                match event {
                    Event::Key(KeyEvent { code, .. }) => match code {
                        KeyCode::Backspace => {
                            event_loop::Event::Key(event_loop::Key::Backspace)
                        }
                        KeyCode::Char(ch) => {
                            event_loop::Event::Key(event_loop::Key::Char(ch))
                        }
                        _ => continue,
                    },
                    _ => continue,
                }
            }
            None => event_loop::Event::Tick,
        };

        event_loop::run_once(event, &mut state)?;
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
