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
        registry.define("empty_generation", "[ { false } num_cells times ]");
        registry.define("times", "done? { clean_up } { loop again } if");
        registry.define("done?", "clone 0 =");
        registry.define("clean_up", "drop drop");
        registry.define("loop", "over eval reset_stack");
        registry.define("reset_stack", "rot rot");
        registry.define("again", "1 - times");
        registry.define(
            "init",
            "empty_generation \
                37 true list_set \
                38 true list_set \
                39 true list_set \
                41 true list_set \
                42 true list_set \
                43 true list_set",
        );
        registry.define(
            "neighbor_range",
            "clone neighbor_range_min swap neighbor_range_max",
        );
        registry.define("neighbor_range_min", "2 -");
        registry.define("neighbor_range_max", "2 + num_cells 1 - min");
        registry.define("num_cells", "80");
        registry.define("cell_is_alive", "list_get");
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
