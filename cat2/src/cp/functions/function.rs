use crate::cp::{parse, tokenize, Expressions, Type};

pub struct Function {
    pub name: String,
    pub args: Args,
    pub body: Expressions,
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
            body: parse(tokenize(body)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Arg {
    Type(Type),
}

impl Arg {
    pub fn ty(&self) -> Type {
        match self {
            Arg::Type(ty) => ty.clone(),
        }
    }

    pub fn value(&self) -> Option<&crate::cp::Value> {
        match self {
            Arg::Type(_) => None,
        }
    }

    pub fn is_type(&self) -> bool {
        match self {
            Self::Type(_) => true,
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            Self::Type(_) => false,
        }
    }
}
