mod cells;
mod cp;
mod ui;

use std::{
    collections::VecDeque,
    io::stdout,
    thread,
    time::{Duration, Instant},
};

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout();
    let mut buffer = ui::Buffer::new();
    let mut lines = VecDeque::new();

    let mut time = Instant::now();
    let delay = Duration::from_millis(125);

    loop {
        let current = lines.back().cloned().unwrap_or_else(cells::init);

        let next = cells::next_generation(current);
        lines.push_back(next);

        ui::draw(&mut lines, &mut buffer, &mut stdout)?;

        thread::sleep(delay - time.elapsed());
        time += delay;
    }
}
