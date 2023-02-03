mod cells;
mod cp;
mod ui;

use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let mut current = ui::Line::init();

    loop {
        let mut next = ui::Line::empty();

        for (i, cell) in next.inner.iter_mut().enumerate() {
            let (min, max) = cells::neighbor_range(i as u8);
            let (min, max) = (min as usize, max as usize);

            let mut num_neighbors = 0;
            (min..=max).for_each(|j| {
                if current.inner[j] && i != j {
                    num_neighbors += 1;
                }
            });

            *cell = cells::cell_lives(current.inner[i], num_neighbors);
        }

        current = next;
        current.print()?;

        let now = Instant::now();
        while now.elapsed().as_secs_f64() < 0.125 {}
    }
}
