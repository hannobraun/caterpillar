mod cells;
mod cp;
mod ui;

use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let mut current = ui::Line::init();

    loop {
        let mut next = ui::Line::empty();
        next.inner = cells::next_generation(current.inner);

        current = next;
        current.print()?;

        let now = Instant::now();
        while now.elapsed().as_secs_f64() < 0.125 {}
    }
}
