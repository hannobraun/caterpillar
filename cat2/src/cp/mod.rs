mod builtins;
mod data_stack;
mod functions;
mod tokenizer;

pub use self::{
    data_stack::{DataStack, Type, Value},
    functions::{Arg, Functions},
    tokenizer::{tokenize, Tokens},
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
    let function = functions
        .resolve(fn_name, data_stack)
        .ok_or(FunctionNotFound)?;

    for token in &function.tokens {
        evaluate(token, functions, data_stack)?;
    }

    Ok(())
}

#[derive(Debug)]
pub struct FunctionNotFound;
