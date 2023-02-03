mod cells;
mod cp;
mod ui;

use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let mut current = ui::Line {
        inner: cells::init(),
    };

    loop {
        let mut next = ui::Line {
            inner: [false; cells::NUM_CELLS],
        };

        for (i, cell) in next.inner.iter_mut().enumerate() {
            let min = if i > 2 { i - 2 } else { 0 };
            let max = if i < current.inner.len() - 1 - 2 {
                i + 2
            } else {
                current.inner.len() - 1
            };

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
