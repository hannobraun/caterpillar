use crate::cp::{parse, tokenize, Expressions};

pub struct Function {
    pub name: String,
    pub body: Expressions,
}

impl Function {
    pub fn new(name: impl Into<String>, body: &str) -> Self {
        Function {
            name: name.into(),
            body: parse(tokenize(body)),
        }
    }
}
