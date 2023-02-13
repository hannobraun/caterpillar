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
        let num_neighbors = num_neighbors(i as u8, current, interpreter);

        interpreter.data_stack.push(cp::Value::Bool(current[i]));
        interpreter.data_stack.push(cp::Value::U8(num_neighbors));
        cp::evaluate(
            "cell_lives",
            &interpreter.functions,
            &mut interpreter.data_stack,
        )
        .unwrap();
        *cell = interpreter.data_stack.pop_bool();
    }

    next
}

pub fn num_neighbors(
    i: u8,
    cells: Generation,
    interpreter: &mut cp::Interpreter,
) -> u8 {
    let (min, max) = neighbor_range(i, interpreter);

    let mut num_neighbors = 0;
    (min..=max).for_each(|j| {
        if cells[j as usize] && i != j {
            num_neighbors += 1;
        }
    });

    num_neighbors
}

pub fn neighbor_range(i: u8, interpreter: &mut cp::Interpreter) -> (u8, u8) {
    interpreter.data_stack.push(cp::Value::U8(i));
    cp::evaluate(
        "neighbor_range_min",
        &interpreter.functions,
        &mut interpreter.data_stack,
    )
    .unwrap();
    let min = interpreter.data_stack.pop_u8();

    let max = i.saturating_add(2).min(NUM_CELLS as u8 - 1);

    (min, max)
}
