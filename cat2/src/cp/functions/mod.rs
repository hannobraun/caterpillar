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
        registry.define("empty_generation", "[ { false } 80 times ]");
        registry.define("times", "is_done { clean_up } { eval_loop again } if");
        registry.define("is_done", "clone 0 =");
        registry.define("clean_up", "drop drop");
        registry.define("eval_loop", "over eval result_to_bottom");
        registry.define("result_to_bottom", "rot rot");
        registry.define("again", "1 - times");
        registry.define(
            "init",
            "empty_generation \
                37 true set_list \
                38 true set_list \
                39 true set_list \
                41 true set_list \
                42 true set_list \
                43 true set_list",
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
