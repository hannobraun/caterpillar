mod cells;
mod cp;
mod ui;

use std::{io::stdout, thread, time::Duration};

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    let mut buffer = ui::Buffer::new();
    let mut lines = ui::Lines::new();

    loop {
        let current = lines.current();

        let next = cells::next_generation(current.cells());
        let next = ui::Line::from_cells(next);

        lines.push_next(next);
        lines.print(&mut buffer, &mut stdout)?;

        thread::sleep(Duration::from_millis(125));
    }
}
