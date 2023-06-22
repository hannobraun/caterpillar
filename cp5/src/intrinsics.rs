use crate::cp;

pub fn clone(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let a = evaluator.data_stack.pop_any()?;

    evaluator.data_stack.push(a.clone());
    evaluator.data_stack.push(a);

    Ok(())
}

pub fn drop(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    evaluator.data_stack.pop_any()?;
    Ok(())
}

pub fn true_(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    evaluator.data_stack.push(true);
    Ok(())
}

pub fn false_(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    evaluator.data_stack.push(false);
    Ok(())
}

pub fn and(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let a = evaluator.data_stack.pop_bool()?;
    let b = evaluator.data_stack.pop_bool()?;

    evaluator.data_stack.push(a && b);

    Ok(())
}

pub fn not(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let b = evaluator.data_stack.pop_bool()?;
    evaluator.data_stack.push(!b);

    Ok(())
}

pub fn if_(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let else_ = evaluator.data_stack.pop_block()?;
    let then_ = evaluator.data_stack.pop_block()?;
    let cond = evaluator.data_stack.pop_bool()?;

    let block = if cond { then_ } else { else_ };

    evaluator.evaluate_block(block)?;

    Ok(())
}

pub fn unwrap(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let array = evaluator.data_stack.pop_array()?;

    for value in array.elements {
        evaluator.data_stack.push(value);
    }

    Ok(())
}

pub fn eval(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let block = evaluator.data_stack.pop_block()?;
    evaluator.evaluate_block(block)?;
    Ok(())
}

pub fn eq(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let b = evaluator.data_stack.pop_any()?;
    let a = evaluator.data_stack.pop_any()?;

    let eq = a == b;

    evaluator.data_stack.push(eq);

    Ok(())
}

pub fn sub(evaluator: &mut cp::Evaluator) -> Result<(), cp::EvaluatorError> {
    let b = evaluator.data_stack.pop_u8()?;
    let a = evaluator.data_stack.pop_u8()?;

    evaluator.data_stack.push(a - b);

    Ok(())
}
