use crate::cp;

pub const NUM_CELLS: usize = 80;

pub fn init() -> [bool; NUM_CELLS] {
    let mut cells = [false; NUM_CELLS];

    cells[37] = true;
    cells[38] = true;
    cells[39] = true;
    cells[41] = true;
    cells[42] = true;
    cells[43] = true;

    cells
}

pub fn num_neighbors(i: u8, cells: [bool; NUM_CELLS]) -> u8 {
    let (min, max) = neighbor_range(i);

    let mut num_neighbors = 0;
    (min..=max).for_each(|j| {
        if cells[j as usize] && i != j {
            num_neighbors += 1;
        }
    });

    num_neighbors
}

pub fn neighbor_range(i: u8) -> (u8, u8) {
    let min = i.saturating_sub(2);
    let max = i.saturating_add(2).min(NUM_CELLS as u8 - 1);

    (min, max)
}

pub fn cell_lives(lives_already: bool, num_neighbors: u8) -> bool {
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
