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

pub fn interpret(fn_name: &str, interpreter: &mut Interpreter) {
    let function = interpreter
        .functions
        .find(fn_name, &interpreter.data_stack)
        .unwrap();

    for token in &function.tokens {
        match token.as_str() {
            "clone" => {
                let value = interpreter.data_stack.pop_any();

                interpreter.data_stack.push(value.clone());
                interpreter.data_stack.push(value);
            }
            "or" => {
                let b = interpreter.data_stack.pop_bool();
                let a = interpreter.data_stack.pop_bool();

                interpreter.data_stack.push(a || b);
            }
            "swap" => {
                let b = interpreter.data_stack.pop_any();
                let a = interpreter.data_stack.pop_any();

                interpreter.data_stack.push(b);
                interpreter.data_stack.push(a);
            }
            "=" => {
                let b = interpreter.data_stack.pop_u8();
                let a = interpreter.data_stack.pop_u8();

                interpreter.data_stack.push(a == b);
            }
            token => {
                if let Ok(value) = token.parse::<u8>() {
                    interpreter.data_stack.push(Value::U8(value));
                    continue;
                }

                // If we land here, the token is unknown. We silently swallow
                // that error right now, because we don't have a good way to
                // report it to the user.
            }
        }
    }
}
