mod cells;
mod cp;
mod ui;

use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let mut current = ui::Line::init();

    loop {
        let next = cells::next_generation(current.inner);
        let next = ui::Line::from_cells(next);

        current = next;
        current.print()?;

        let now = Instant::now();
        while now.elapsed().as_secs_f64() < 0.125 {}
    }
}
