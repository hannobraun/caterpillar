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

    let cells = interpreter
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
    assert!(interpreter.data_stack.is_empty());

    cells
}

pub fn next_generation(
    cells: &Generation,
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
    assert!(interpreter.data_stack.is_empty());

    for (i, cell) in next.iter_mut().enumerate() {
        let num_neighbors = count_neighbors(cells, i as u8, interpreter);

        interpreter.data_stack.push(cp::Value::Bool(cells[i]));
        interpreter.data_stack.push(cp::Value::U8(num_neighbors));
        cp::evaluate(
            &vec![cp::Expression::Fn("cell_lives".into())],
            &interpreter.functions,
            &mut interpreter.data_stack,
            &mut interpreter.bindings,
        )
        .unwrap();
        *cell = interpreter.data_stack.pop_bool();
        assert!(interpreter.data_stack.is_empty());
    }

    next
}

pub fn count_neighbors(
    cells: &Generation,
    i: u8,
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
    assert!(interpreter.data_stack.is_empty());

    let mut num_neighbors = 0;
    (min..=max).for_each(|j| {
        interpreter.data_stack.push(cp::Value::List(
            cells.iter().cloned().map(cp::Value::Bool).collect(),
        ));
        interpreter.data_stack.push(cp::Value::U8(i));
        interpreter.data_stack.push(cp::Value::U8(j));
        cp::evaluate(
            &vec![cp::Expression::Fn("count_neighbor".into())],
            &interpreter.functions,
            &mut interpreter.data_stack,
            &mut interpreter.bindings,
        )
        .unwrap();
        num_neighbors += interpreter.data_stack.pop_u8();
        assert!(interpreter.data_stack.is_empty());
    });

    num_neighbors
}
