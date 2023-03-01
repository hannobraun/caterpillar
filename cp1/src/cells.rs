use crate::cp;

pub const NUM_CELLS: usize = 80;

pub type Generation = Vec<bool>;

pub fn init(interpreter: &mut cp::Interpreter) -> Generation {
    cp::evaluate(
        &vec![cp::Expression::Fn("init".into())],
        &interpreter.functions,
        &mut interpreter.data_stack,
        &mut interpreter.bindings,
    )
    .unwrap();

    interpreter
        .data_stack
        .pop_list()
        .into_iter()
        .map(|value| {
            let cp::Value::Bool(value) = value else {
                panic!("Expected `bool`")
            };
            value
        })
        .collect::<Vec<_>>()
}

pub fn next_generation(
    current: &Generation,
    interpreter: &mut cp::Interpreter,
) -> Generation {
    cp::evaluate(
        &vec![cp::Expression::Fn("empty_generation".into())],
        &interpreter.functions,
        &mut interpreter.data_stack,
        &mut interpreter.bindings,
    )
    .unwrap();
    let mut next = interpreter
        .data_stack
        .pop_list()
        .into_iter()
        .map(|value| {
            let cp::Value::Bool(value) = value else {
                panic!("Expected `bool`")
            };
            value
        })
        .collect::<Vec<_>>();

    for (i, cell) in next.iter_mut().enumerate() {
        let num_neighbors = num_neighbors(i as u8, current, interpreter);

        interpreter.data_stack.push(cp::Value::Bool(current[i]));
        interpreter.data_stack.push(cp::Value::U8(num_neighbors));
        cp::evaluate(
            &vec![cp::Expression::Fn("cell_lives".into())],
            &interpreter.functions,
            &mut interpreter.data_stack,
            &mut interpreter.bindings,
        )
        .unwrap();
        *cell = interpreter.data_stack.pop_bool();
    }

    next
}

pub fn num_neighbors(
    i: u8,
    cells: &Generation,
    interpreter: &mut cp::Interpreter,
) -> u8 {
    interpreter.data_stack.push(cp::Value::U8(i));
    cp::evaluate(
        &vec![cp::Expression::Fn("neighbor_range".into())],
        &interpreter.functions,
        &mut interpreter.data_stack,
        &mut interpreter.bindings,
    )
    .unwrap();
    let max = interpreter.data_stack.pop_u8();
    let min = interpreter.data_stack.pop_u8();

    let mut num_neighbors = 0;
    (min..=max).for_each(|j| {
        interpreter.data_stack.push(cp::Value::U8(i));
        interpreter.data_stack.push(cp::Value::U8(j));
        interpreter.data_stack.push(cp::Value::List(
            cells.iter().cloned().map(cp::Value::Bool).collect(),
        ));
        cp::evaluate(
            &vec![cp::Expression::Fn("count_neighbor".into())],
            &interpreter.functions,
            &mut interpreter.data_stack,
            &mut interpreter.bindings,
        )
        .unwrap();
        num_neighbors += interpreter.data_stack.pop_u8();
    });

    num_neighbors
}
