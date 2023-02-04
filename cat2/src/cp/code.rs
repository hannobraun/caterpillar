use std::collections::BTreeMap;

pub struct Code {
    functions: BTreeMap<String, String>,
}

impl Code {
    pub fn new() -> Self {
        let mut functions = BTreeMap::new();

        functions.insert(
            String::from("cell_is_born"),
            String::from(include_str!("../caterpillar/cell_is_born.cp0")),
        );

        Self { functions }
    }

    pub fn function(&self, name: &str) -> &str {
        self.functions
            .get(name)
            .unwrap_or_else(|| panic!("Function {name} not defined"))
    }
}
