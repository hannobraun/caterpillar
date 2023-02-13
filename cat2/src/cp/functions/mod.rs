mod function;
mod registry;

pub use self::function::{Arg, Args};

use self::{function::Function, registry::Registry};

use super::{DataStack, Type, Value};

pub struct Functions {
    registry: Registry,
}

impl Functions {
    pub fn new() -> Self {
        let registry = Registry::new();
        let mut self_ = Self { registry };

        // Eventually, we'll store the source code in a persistent way. But for
        // now, we'll just define default code on startup, as a starting point
        // for the user to modify.
        self_.define(
            "cell_lives",
            [Arg::Value(Value::Bool(true)), Arg::Type(Type::U8)],
            "swap drop cell_survives",
        );
        self_.define(
            "cell_lives",
            [Arg::Value(Value::Bool(false)), Arg::Type(Type::U8)],
            "swap drop cell_is_born",
        );
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
        self.registry.define(name, args, body);
    }

    pub fn find(&self, name: &str, stack: &DataStack) -> Option<&Function> {
        self.registry
            .resolve(name, stack.values_from_top().cloned())
    }

    pub fn get(&self, name: &str, args: impl Into<Args>) -> Option<&Function> {
        self.registry.get(name, args)
    }

    pub fn get_mut(
        &mut self,
        name: &str,
        args: impl Into<Args>,
    ) -> Option<&mut Function> {
        self.registry.get_mut(name, args)
    }
}
