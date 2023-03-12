use crate::cp;

pub const NUM_CELLS: usize = 80;

pub type Generation = Vec<bool>;

pub fn init(interpreter: &mut cp::Interpreter) -> Generation {
    cp::evaluate(
        &cp::Expressions {
            inner: vec![cp::Expression::Fn("init".into())],
        },
        &interpreter.functions,
        &mut interpreter.data_stack,
        &mut interpreter.bindings,
        false,
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
        &cp::Expressions {
            inner: vec![cp::Expression::Fn("empty_generation".into())],
        },
        &interpreter.functions,
        &mut interpreter.data_stack,
        &mut interpreter.bindings,
        false,
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

    let mut i = 0;
    loop {
        if i >= next.len() {
            break;
        }

        interpreter.data_stack.push(cp::Value::List(
            cells.iter().cloned().map(cp::Value::Bool).collect(),
        ));
        interpreter.data_stack.push(cp::Value::List(
            next.iter().cloned().map(cp::Value::Bool).collect(),
        ));
        interpreter.data_stack.push(cp::Value::U8(i as u8));
        cp::evaluate(
            &cp::Expressions {
                inner: vec![cp::Expression::Fn("next_generation_cell".into())],
            },
            &interpreter.functions,
            &mut interpreter.data_stack,
            &mut interpreter.bindings,
            false,
        )
        .unwrap();
        next = interpreter
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

        i += 1;
    }

    next
}
