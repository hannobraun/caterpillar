use std::collections::BTreeMap;

pub struct Code {
    pub inner: BTreeMap<String, String>,
}

impl Code {
    pub fn new() -> Self {
        let mut inner = BTreeMap::new();

        inner.insert(
            String::from("cell_is_born"),
            String::from(include_str!("../caterpillar/cell_is_born.cp0")),
        );

        Self { inner }
    }
}
