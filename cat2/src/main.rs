mod cells;
mod cp;
mod ui;

use std::{io::stdout, time::Duration};

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
    let mut buffer = ui::Buffer::new();
    let mut stdout = stdout();
    let mut events = EventStream::new();

    terminal::enable_raw_mode()?;

    let mut generations = Vec::new();

    let functions = cp::Functions::new();

    let delay = Duration::from_millis(125);
    let mut interval = time::interval(delay);
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = interval.tick() => {}
            event = events.next() => {
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
        }

        let current = generations.last().cloned().unwrap_or_else(cells::init);

        // We only add new generations, but never delete them. This is fine for
        // now, I think. Let's just hope nobody runs this for long enough to
        // fill up their main memory.
        let next = cells::next_generation(current, &functions);
        generations.push(next);

        ui::draw(&generations, &functions, &mut buffer, &mut stdout)?;
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
