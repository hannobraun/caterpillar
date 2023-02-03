mod cp;

use std::time::Instant;

const NUM_CELLS: usize = 80;

fn main() {
    let mut current = [false; NUM_CELLS];

    // Initial conditions.
    current[37] = true;
    current[38] = true;
    current[39] = true;
    current[41] = true;
    current[42] = true;
    current[43] = true;

    loop {
        print!("┃");
        for &cell in &current {
            if cell {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("┃");

        let mut next = [false; NUM_CELLS];

        for i in 0..next.len() {
            let min = if i > 2 { i - 2 } else { 0 };
            let max = if i < current.len() - 1 - 2 {
                i + 2
            } else {
                current.len() - 1
            };

            let mut num_neighbors = 0;
            (min..=max).for_each(|j| {
                if current[j] && i != j {
                    num_neighbors += 1;
                }
            });

            next[i] = cell_lives(current[i], num_neighbors);
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
