use std::time::Instant;

fn main() {
    let mut current = [false; 80];

    // Initial conditions.
    current[37] = true;
    current[38] = true;
    current[39] = true;
    current[41] = true;
    current[42] = true;
    current[43] = true;

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
                if current[j] {
                    num_neighbors += 1;
                }
            });

            if current[i] {
                next[i] = num_neighbors == 2 || num_neighbors == 4;
            } else {
                next[i] = num_neighbors == 2 || num_neighbors == 3;
            }
        }

        current = next;

        let now = Instant::now();
        while now.elapsed().as_secs_f64() < 0.5 {}
    }
}
