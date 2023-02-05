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
    let mut lines = ui::Lines::new();

    let mut time = Instant::now();
    let delay = Duration::from_millis(125);

    loop {
        let current = lines.current();

        let next = cells::next_generation(current.cells());
        let next = ui::Line::from_cells(next);

        lines.push_next(next);
        lines.print(&mut buffer, &mut stdout)?;

        thread::sleep(delay - time.elapsed());
        time += delay;
    }
}
