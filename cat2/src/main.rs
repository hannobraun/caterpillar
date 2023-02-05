mod cells;
mod cp;
mod ui;

use std::{io::stdout, time::Instant};

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    let mut lines = ui::Lines::new();

    loop {
        let current = lines.current();

        let next = cells::next_generation(current.cells());
        let next = ui::Line::from_cells(next);

        lines.push_next(next);
        lines.print(&mut stdout)?;

        // I wrote this in a moment of idiocy, when I had temporarily forgotten
        // that `thread::sleep` exists. However, trying to replace it with that
        // function call completely messes up the UI. It starts flickering, but
        // with this busy loop here, it's perfectly smooth for some reason.
        let now = Instant::now();
        while now.elapsed().as_secs_f64() < 0.125 {}
    }
}
