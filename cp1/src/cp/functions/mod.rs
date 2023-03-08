mod function;
mod registry;

use self::{function::Function, registry::Registry};

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
        registry.define(
            "times",
            "[ :block :num ] bind
                num done?
                { }
                {
                    block eval
                    block num again
                }
                    if",
        );
        registry.define("done?", "0 =");
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
            "next_generation_cell",
            "[ :cells :next_cells :i ] bind
                cells i count_neighbors
                    [ :num_neighbors ] bind
                cells i num_neighbors cell_lives
                    [ :is_alive ] bind
                next_cells i is_alive list_set",
        );
        registry.define(
            "count_neighbors",
            "[ :cells :i ] bind
                i neighbor_range
                    [ :min_index :max_index ] bind
                cells min_index max_index i count_each_neighbor",
        );
        registry.define(
            "neighbor_range",
            "[ :i ] bind
                i neighbor_range_min
                i neighbor_range_max",
        );
        registry.define("neighbor_range_min", "2 -");
        registry.define(
            "neighbor_range_max",
            "
            2 +
            num_cells 1 -
                min",
        );
        registry.define("num_cells", "80");
        registry.define(
            "count_each_neighbor",
            "[ :cells :min_index :max_index :i ] bind
                min_index 0
                {
                    [ :j :count ] bind
                    cells i j count_neighbor
                        count + [ :count ] bind
                    j 1 + [ :j ] bind
                    j count
                }
                max_index min_index - 1 +
                    times
                swap drop",
        );
        registry.define(
            "count_neighbor",
            "[ :cells :i :j ] bind
                cells j cell_is_alive swap drop
                i j cell_is_neighbor
                    and { 1 } { 0 } if",
        );
        registry.define("cell_is_alive", "list_get");
        registry.define("cell_is_neighbor", "= not");
        registry.define(
            "cell_lives",
            "[ :cells :i :num_neighbors ] bind
                cells i list_get
                    [ :is_alive ] bind
                    drop
                num_neighbors
                    is_alive { cell_survives } { cell_is_born } if",
        );
        registry.define(
            "cell_is_born",
            "[ :num_neighbors ] bind
                num_neighbors 2 =
                num_neighbors 3 =
                    or",
        );
        registry.define(
            "cell_survives",
            "[ :num_neighbors ] bind
                num_neighbors 2 =
                num_neighbors 4 =
                    or",
        );

        Self { registry }
    }

    pub fn resolve(&self, name: &str) -> Option<&Function> {
        self.registry.resolve(name)
    }
}
