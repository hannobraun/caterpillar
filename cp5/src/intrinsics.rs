use crate::cp;

pub fn define(functions: &mut cp::Functions) {
    let intrinsics = [
        ("clone", clone as cp::Intrinsic),
        ("drop", drop),
        ("true", true_),
        ("false", false_),
        ("and", and),
        ("not", not),
        ("if", if_),
        ("unwrap", unwrap),
        ("eval", eval),
        ("=", eq),
        ("-", sub),
    ];

    for (name, body) in intrinsics {
        functions.register_intrinsic(cp::Module::none(), name.into(), body);
    }
}

fn clone(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let a = evaluator.data_stack.pop_any()?;

    evaluator.data_stack.push(a.clone());
    evaluator.data_stack.push(a);

    Ok(())
}

fn drop(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    evaluator.data_stack.pop_any()?;
    Ok(())
}

fn true_(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    evaluator.data_stack.push(true);
    Ok(())
}

fn false_(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    evaluator.data_stack.push(false);
    Ok(())
}

fn and(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let a = evaluator.data_stack.pop_bool()?;
    let b = evaluator.data_stack.pop_bool()?;

    evaluator.data_stack.push(a && b);

    Ok(())
}

fn not(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let b = evaluator.data_stack.pop_bool()?;
    evaluator.data_stack.push(!b);

    Ok(())
}

fn if_(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let else_ = evaluator.data_stack.pop_block()?;
    let then_ = evaluator.data_stack.pop_block()?;
    let cond = evaluator.data_stack.pop_bool()?;

    let block = if cond { then_ } else { else_ };

    evaluator.evaluate_expressions(&block)?;

    Ok(())
}

fn unwrap(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let array = evaluator.data_stack.pop_array()?;

    for value in array.elements {
        evaluator.data_stack.push(value);
    }

    Ok(())
}

fn eval(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let block = evaluator.data_stack.pop_block()?;
    evaluator.evaluate_expressions(&block)?;
    Ok(())
}

fn eq(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let b = evaluator.data_stack.pop_any()?;
    let a = evaluator.data_stack.pop_any()?;

    let eq = a == b;

    evaluator.data_stack.push(eq);

    Ok(())
}

fn sub(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let b = evaluator.data_stack.pop_u8()?;
    let a = evaluator.data_stack.pop_u8()?;

    evaluator.data_stack.push(a - b);

    Ok(())
}
