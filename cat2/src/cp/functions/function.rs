use crate::cp::{tokenize, Tokens, Type};

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

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Args {
    pub inner: Vec<Arg>,
}

impl<T> From<T> for Args
where
    T: IntoIterator<Item = Arg>,
{
    fn from(iter: T) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Arg {
    Type(Type),
}
