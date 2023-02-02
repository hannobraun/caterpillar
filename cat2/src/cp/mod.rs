mod stack;

pub use self::stack::{Stack, Value};

pub fn interpret(code: &str, stack: &mut Stack) {
    let tokens = tokenize(code);
    evaluate(tokens, stack);
}

fn tokenize(code: &str) -> impl Iterator<Item = &str> {
    code.split_whitespace()
}

fn evaluate<'a>(tokens: impl Iterator<Item = &'a str>, stack: &mut Stack) {
    for token in tokens {
        match token {
            "clone" => {
                let Some(value) = stack.inner.pop() else {
                    panic!("Expected value")
                };
                stack.inner.push(value.clone());
                stack.inner.push(value);
            }
            "or" => {
                let Some(Value::Bool(b)) = stack.inner.pop() else {
                    panic!("Expected `bool`")
                };
                let Some(Value::Bool(a)) = stack.inner.pop() else {
                    panic!("Expected `bool`")
                };

                let value = Value::Bool(a || b);
                stack.inner.push(value);
            }
            "swap" => {
                let Some(b) = stack.inner.pop() else {
                    panic!("Expected value")
                };
                let Some(a) = stack.inner.pop() else {
                    panic!("Expected value")
                };

                stack.inner.push(b);
                stack.inner.push(a);
            }
            "=" => {
                let Some(Value::U8(b)) = stack.inner.pop() else {
                    panic!("Expected `u8`")
                };
                let Some(Value::U8(a)) = stack.inner.pop() else {
                    panic!("Expected `u8`")
                };

                let value = Value::Bool(a == b);
                stack.inner.push(value);
            }
            token => {
                if let Ok(value) = token.parse::<u8>() {
                    stack.inner.push(Value::U8(value));
                    continue;
                }

                panic!("Unknown token: `{token}`")
            }
        }
    }
}
