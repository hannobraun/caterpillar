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
    cells: Generation,
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
        interpreter.data_stack.push(cp::Value::List(
            cells.iter().cloned().map(cp::Value::Bool).collect(),
        ));
        interpreter.data_stack.push(cp::Value::U8(i as u8));
        cp::evaluate(
            &vec![cp::Expression::Fn("next_generation_cell".into())],
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
