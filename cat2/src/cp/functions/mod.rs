mod function;
mod registry;

use self::{function::Function, registry::Registry};

use super::DataStack;

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
            "empty_generation",
            "[ false false false false false false false false \
            false false false false false false false false \
            false false false false false false false false \
            false false false false false false false false \
            false false false false false false false false \
            false false false false false false false false \
            false false false false false false false false \
            false false false false false false false false \
            false false false false false false false false \
            false false false false false false false false ]",
        );
        registry.define(
            "neighbor_range",
            "clone neighbor_range_min swap neighbor_range_max",
        );
        registry.define("neighbor_range_min", "2 -");
        registry.define("neighbor_range_max", "2 + num_cells 1 - min");
        registry.define("num_cells", "80");
        registry
            .define("cell_lives", "swap { cell_survives } { cell_is_born } if");

        registry.define("cell_is_born", "clone 2 = swap 3 = or");
        registry.define("cell_survives", "clone 2 = swap 4 = or");

        Self { registry }
    }

    pub fn resolve(&self, name: &str, _: &DataStack) -> Option<&Function> {
        self.registry.resolve(name)
    }
}
