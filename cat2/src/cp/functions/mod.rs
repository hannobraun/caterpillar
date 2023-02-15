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
        let mut registry = Registry::new();

        // Eventually, we'll store the source code in a persistent way. But for
        // now, we'll just define default code on startup, as a starting point
        // for the user to modify.
        registry.define(
            "neighbor_range",
            [Arg::Type(Type::U8)],
            "clone neighbor_range_min swap neighbor_range_max",
        );
        registry.define("neighbor_range_min", [Arg::Type(Type::U8)], "2 -");
        registry.define(
            "neighbor_range_max",
            [Arg::Type(Type::U8)],
            "2 + num_cells 1 - min",
        );
        registry.define("num_cells", [], "80");
        registry.define(
            "cell_lives",
            [Arg::Value(Value::Bool(true)), Arg::Type(Type::U8)],
            "swap drop cell_survives",
        );
        registry.define(
            "cell_lives",
            [Arg::Value(Value::Bool(false)), Arg::Type(Type::U8)],
            "swap drop cell_is_born",
        );
        registry.define(
            "cell_is_born",
            [Arg::Type(Type::U8)],
            "clone 2 = swap 3 = or",
        );
        registry.define(
            "cell_survives",
            [Arg::Type(Type::U8)],
            "clone 2 = swap 4 = or",
        );

        Self { registry }
    }

    pub fn resolve(&self, name: &str, stack: &DataStack) -> Option<&Function> {
        self.registry
            .resolve(name, stack.values_from_top().cloned())
    }

    pub fn get_mut(
        &mut self,
        name: &str,
        args: impl Into<Args>,
    ) -> Option<&mut Function> {
        self.registry.get_mut(name, args)
    }
}
