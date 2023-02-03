mod cells;
mod cp;
mod ui;

use std::{iter, time::Instant};

use crossterm::terminal;

fn main() -> anyhow::Result<()> {
    let mut current = ui::Line {
        inner: cells::init(),
    };

    loop {
        let (num_columns, _) = terminal::size()?;
        iter::repeat(' ')
            .take(num_columns as usize - cells::NUM_CELLS - 2)
            .for_each(|c| print!("{c}"));

        print!("┃");
        for &cell in &current.inner {
            if cell {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("┃");

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

            *cell = cell_lives(current.inner[i], num_neighbors);
        }

        current = next;

        let now = Instant::now();
        while now.elapsed().as_secs_f64() < 0.125 {}
    }
}

fn cell_lives(lives_already: bool, num_neighbors: u8) -> bool {
    if lives_already {
        cell_survives(num_neighbors)
    } else {
        cell_is_born(num_neighbors)
    }
}

fn cell_survives(num_neighbors: u8) -> bool {
    num_neighbors == 2 || num_neighbors == 4
}

fn cell_is_born(num_neighbors: u8) -> bool {
    let code = include_str!("caterpillar/cell_is_born.cp0");
    let mut stack = cp::Stack::from_values(&[cp::Value::U8(num_neighbors)]);
    cp::interpret(code, &mut stack);
    stack.pop_bool()
}
