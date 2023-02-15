use crate::cp::{tokenize, Tokens, Type};

pub struct Function {
    pub name: String,
    pub args: Args,
    pub body: Tokens,
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
            body: tokenize(body),
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

    #[cfg_attr(not(test), allow(dead_code))]
    Value(crate::cp::Value),
}

impl Arg {
    pub fn ty(&self) -> Type {
        match self {
            Arg::Type(ty) => ty.clone(),
            Arg::Value(value) => value.ty(),
        }
    }

    pub fn value(&self) -> Option<&crate::cp::Value> {
        match self {
            Arg::Type(_) => None,
            Arg::Value(value) => Some(value),
        }
    }

    pub fn is_type(&self) -> bool {
        match self {
            Self::Type(_) => true,
            Self::Value(_) => false,
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            Self::Type(_) => false,
            Self::Value(_) => true,
        }
    }
}
