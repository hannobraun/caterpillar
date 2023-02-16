use super::{builtins, DataStack, Expression, Expressions, Functions, Value};

pub fn evaluate(
    expressions: &Expressions,
    functions: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    for expression in expressions {
        evaluate_expression(expression, functions, data_stack)?;
    }

    Ok(())
}

fn evaluate_expression(
    expression: &Expression,
    functions: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    match expression {
        Expression::Block(expressions) => {
            data_stack.push(Value::Block(expressions.clone()));
            Ok(())
        }
        Expression::Fn(fn_name) => evaluate_fn(fn_name, functions, data_stack),
    }
}

fn evaluate_fn(
    fn_name: &str,
    functions: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    if let Some(builtin) = builtins::get(fn_name) {
        builtin(functions, data_stack)?;
        return Ok(());
    }

    if let Ok(value) = fn_name.parse::<u8>() {
        data_stack.push(Value::U8(value));
        return Ok(());
    }

    // If we land here, it's not a builtin function.
    let function = functions.resolve(fn_name, data_stack).ok_or_else(|| {
        FunctionNotFound {
            name: fn_name.into(),
        }
    })?;

    evaluate(&function.body, functions, data_stack)?;

    Ok(())
}

#[derive(Debug)]
pub struct FunctionNotFound {
    pub name: String,
}
