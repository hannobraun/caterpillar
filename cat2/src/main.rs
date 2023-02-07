mod cells;
mod cp;
mod ui;

use std::{io::stdout, time::Duration};

use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut buffer = ui::Buffer::new();
    let mut stdout = stdout();

    let mut generations = Vec::new();

    let functions = cp::Functions::new();

    let delay = Duration::from_millis(125);
    let mut interval = time::interval(delay);
    interval.tick().await; // the first tick is immediate; get it out of the way

    loop {
        let current = generations.last().cloned().unwrap_or_else(cells::init);

        // We only add new generations, but never delete them. This is fine for
        // now, I think. Let's just hope nobody runs this for long enough to
        // fill up their main memory.
        let next = cells::next_generation(current, &functions);
        generations.push(next);

        ui::draw(&generations, &functions, &mut buffer, &mut stdout)?;

        interval.tick().await;
    }
}
