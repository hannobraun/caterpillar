mod functions;
mod stack;
mod tokenizer;

pub use self::{
    functions::Functions,
    stack::{DataStack, Value},
    tokenizer::tokenize,
};

pub struct Interpreter {
    pub data_stack: DataStack,
}

pub fn interpret(code: &str, interpreter: &mut Interpreter) {
    let tokens = tokenize(code);
    evaluate(tokens, &mut interpreter.data_stack);
}

fn evaluate<'a>(tokens: impl Iterator<Item = &'a str>, stack: &mut DataStack) {
    for token in tokens {
        match token {
            "clone" => {
                let value = stack.pop_any();

                stack.push(value.clone());
                stack.push(value);
            }
            "or" => {
                let b = stack.pop_bool();
                let a = stack.pop_bool();

                stack.push(a || b);
            }
            "swap" => {
                let b = stack.pop_any();
                let a = stack.pop_any();

                stack.push(b);
                stack.push(a);
            }
            "=" => {
                let b = stack.pop_u8();
                let a = stack.pop_u8();

                stack.push(a == b);
            }
            token => {
                if let Ok(value) = token.parse::<u8>() {
                    stack.push(Value::U8(value));
                    continue;
                }

                // If we land here, the token is unknown. We silently swallow
                // that error right now, because we don't have a good way to
                // report it to the user.
            }
        }
    }
}
