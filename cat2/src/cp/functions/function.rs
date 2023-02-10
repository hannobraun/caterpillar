use crate::cp::{tokenize, Tokens};

pub struct Function {
    pub name: String,
    pub tokens: Tokens,
}

impl Function {
    pub fn new(name: impl Into<String>, body: &str) -> Self {
        Function {
            name: name.into(),
            tokens: tokenize(body),
        }
    }
}
