mod cells;
mod cp;
mod ui;

use std::{
    io::stdout,
    thread,
    time::{Duration, Instant},
};

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    let mut buffer = ui::Buffer::new();
    let mut lines = Vec::new();

    let mut time = Instant::now();
    let delay = Duration::from_millis(125);

    loop {
        let current = lines.last().cloned().unwrap_or_else(cells::init);

        // We only add new generations, but never delete them. This is fine for
        // now, I think. Let's just hope nobody runs this for long enough to
        // fill up their main memory.
        let next = cells::next_generation(current);
        lines.push(next);

        ui::draw(&lines, &mut buffer, &mut stdout)?;

        thread::sleep(delay - time.elapsed());
        time += delay;
    }
}
