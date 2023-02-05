use std::collections::BTreeMap;

pub struct Functions {
    functions: BTreeMap<String, String>,
}

impl Functions {
    pub fn new() -> Self {
        let mut functions = BTreeMap::new();

        // Eventually, we'll store the source code in a persistent way. But for
        // now, we'll just define default code on startup, as a starting point
        // for the user to modify.
        functions.insert(
            String::from("cell_is_born"),
            String::from("clone 2 = swap 3 = or"),
        );

        Self { functions }
    }

    pub fn function(&self, name: &str) -> &str {
        self.functions
            .get(name)
            .unwrap_or_else(|| panic!("Function {name} not defined"))
    }
}
