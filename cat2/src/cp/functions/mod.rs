mod registry;

use std::collections::BTreeMap;

use self::registry::Registry;

use super::{tokenize, DataStack, Tokens, Type};

pub struct Functions {
    registry: Registry,
}

impl Functions {
    pub fn new() -> Self {
        let registry = BTreeMap::new();
        let mut self_ = Self { registry };

        // Eventually, we'll store the source code in a persistent way. But for
        // now, we'll just define default code on startup, as a starting point
        // for the user to modify.
        self_.define(
            "cell_is_born",
            [Arg::Type(Type::U8)],
            "clone 2 = swap 3 = or",
        );
        self_.define(
            "cell_survives",
            [Arg::Type(Type::U8)],
            "clone 2 = swap 4 = or",
        );

        self_
    }

    pub fn define(
        &mut self,
        name: impl Into<String>,
        args: impl Into<Args>,
        body: &str,
    ) {
        self.registry.insert(
            (name.into(), args.into()),
            Function {
                tokens: tokenize(body),
            },
        );
    }

    pub fn get(
        &self,
        name: &str,
        args: impl IntoIterator<Item = Arg>,
    ) -> Option<&Function> {
        self.registry.get(&(name.into(), args.into()))
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.registry
            .get_mut(&(name.into(), [Arg::Type(Type::U8)].into()))
    }

    pub fn find(&self, name: &str, stack: &DataStack) -> Option<&Function> {
        let mut args = Vec::new();
        let mut values = stack.values_from_top();

        loop {
            if let Some(function) = self.get(name, args.iter().rev().cloned()) {
                return Some(function);
            }

            let value = values.next()?;
            args.push(Arg::Type(value.ty()));
        }
    }
}

pub struct Function {
    pub tokens: Tokens,
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
