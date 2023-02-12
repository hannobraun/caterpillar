use crate::cp;

pub const NUM_CELLS: usize = 80;

pub type Generation = [bool; NUM_CELLS];

pub fn init() -> Generation {
    let mut cells = [false; NUM_CELLS];

    cells[37] = true;
    cells[38] = true;
    cells[39] = true;
    cells[41] = true;
    cells[42] = true;
    cells[43] = true;

    cells
}

pub fn next_generation(
    current: Generation,
    interpreter: &mut cp::Interpreter,
) -> Generation {
    let mut next = [false; NUM_CELLS];

    for (i, cell) in next.iter_mut().enumerate() {
        let num_neighbors = num_neighbors(i as u8, current);
        *cell = cell_lives(current[i], num_neighbors, interpreter);
    }

    next
}

pub fn num_neighbors(i: u8, cells: Generation) -> u8 {
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

pub fn cell_lives(
    lives_already: bool,
    num_neighbors: u8,
    interpreter: &mut cp::Interpreter,
) -> bool {
    interpreter.data_stack.push(cp::Value::U8(num_neighbors));

    if lives_already {
        cp::evaluate("cell_survives", interpreter);
    } else {
        cp::evaluate("cell_is_born", interpreter);
    }

    interpreter.data_stack.pop_bool()
}
