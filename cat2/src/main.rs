mod cells;
mod cp;
mod ui;

use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let mut current = ui::Line::init();

    loop {
        let mut next = ui::Line::empty();

        for (i, cell) in next.inner.iter_mut().enumerate() {
            let num_neighbors = cells::num_neighbors(i as u8, current.inner);
            *cell = cells::cell_lives(current.inner[i], num_neighbors);
        }

        current = next;
        current.print()?;

        let now = Instant::now();
        while now.elapsed().as_secs_f64() < 0.125 {}
    }
}
