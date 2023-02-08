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
    functions: &cp::Functions,
) -> Generation {
    let mut next = [false; NUM_CELLS];

    for (i, cell) in next.iter_mut().enumerate() {
        let num_neighbors = num_neighbors(i as u8, current);
        *cell = cell_lives(current[i], num_neighbors, interpreter, functions);
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
    functions: &cp::Functions,
) -> bool {
    if lives_already {
        cell_survives(num_neighbors)
    } else {
        interpreter.data_stack.push(cp::Value::U8(num_neighbors));
        cp::interpret(functions.get("cell_is_born"), interpreter);
        interpreter.data_stack.pop_bool()
    }
}

fn cell_survives(num_neighbors: u8) -> bool {
    num_neighbors == 2 || num_neighbors == 4
}
