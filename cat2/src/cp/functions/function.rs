use crate::cp::{tokenize, Tokens};

pub struct Function {
    pub tokens: Tokens,
}

impl Function {
    pub fn new(body: &str) -> Self {
        Function {
            tokens: tokenize(body),
        }
    }
}
