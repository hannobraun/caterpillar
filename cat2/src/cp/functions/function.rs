use crate::cp::{tokenize, Tokens, Type};

pub struct Function {
    pub name: String,
    pub args: Args,
    pub tokens: Tokens,
}

impl Function {
    pub fn new(
        name: impl Into<String>,
        args: impl Into<Args>,
        body: &str,
    ) -> Self {
        Function {
            name: name.into(),
            args: args.into(),
            tokens: tokenize(body),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
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
