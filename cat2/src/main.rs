use std::time::Instant;

fn main() {
    let mut current = [false; 80];

    // Initial conditions.
    current[40] = true;
    current[41] = true;

    loop {
        for &cell in &current {
            if cell {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();

        let mut next = [false; 80];

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

            if current[i] {
                next[i] = cell_survives(num_neighbors);
            } else {
                next[i] = cell_is_born(num_neighbors);
            }
        }

        current = next;

        let now = Instant::now();
        while now.elapsed().as_secs_f64() < 0.125 {}
    }
}

fn cell_survives(num_neighbors: u8) -> bool {
    num_neighbors == 2 || num_neighbors == 4
}

fn cell_is_born(num_neighbors: u8) -> bool {
    num_neighbors == 2 || num_neighbors == 3
}
