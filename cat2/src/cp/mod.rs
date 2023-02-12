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
    match fn_name {
        "clone" => {
            let value = data_stack.pop_any();

            data_stack.push(value.clone());
            data_stack.push(value);
        }
        "drop" => {
            data_stack.pop_any();
        }
        "or" => {
            let b = data_stack.pop_bool();
            let a = data_stack.pop_bool();

            data_stack.push(a || b);
        }
        "swap" => {
            let b = data_stack.pop_any();
            let a = data_stack.pop_any();

            data_stack.push(b);
            data_stack.push(a);
        }
        "=" => {
            let b = data_stack.pop_u8();
            let a = data_stack.pop_u8();

            data_stack.push(a == b);
        }
        token => {
            if let Ok(value) = token.parse::<u8>() {
                data_stack.push(Value::U8(value));
                return Ok(());
            }

            // If we land here, it's not a builtin function.
            let function = functions
                .find(fn_name, data_stack)
                .ok_or(FunctionNotFound)?;

            for token in &function.tokens {
                evaluate(token, functions, data_stack)?;
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct FunctionNotFound;
