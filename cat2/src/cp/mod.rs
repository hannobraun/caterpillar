mod builtins;
mod data_stack;
mod functions;
mod parser;
mod tokenizer;

pub use self::{
    data_stack::{DataStack, Type, Value},
    functions::{Arg, Functions},
    parser::{parse, Expression, Expressions},
    tokenizer::{tokenize, Token, Tokens},
};

pub struct Interpreter {
    pub functions: Functions,
    pub data_stack: DataStack,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            functions: Functions::new(),
            data_stack: DataStack::new(),
        }
    }
}

pub fn evaluate(
    expression: &Expression,
    functions: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    match expression {
        Expression::Block(_) => {
            // Currently ignored.
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
        builtin(data_stack);
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

    for expression in &function.body {
        evaluate(expression, functions, data_stack)?;
    }

    Ok(())
}

#[derive(Debug)]
pub struct FunctionNotFound {
    pub name: String,
}
