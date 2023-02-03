mod cells;
mod cp;
mod ui;

use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let mut lines = ui::Lines::new();

    loop {
        let current =
            lines.inner.back().cloned().unwrap_or_else(ui::Line::init);

        let next = cells::next_generation(current.cells);
        let next = ui::Line::from_cells(next);

        next.print()?;
        lines.inner.push_back(next);

        let now = Instant::now();
        while now.elapsed().as_secs_f64() < 0.125 {}
    }
}
