pub struct Code {
    pub inner: String,
}

impl Code {
    pub fn new() -> Self {
        let inner =
            String::from(include_str!("../caterpillar/cell_is_born.cp0"));
        Self { inner }
    }
}
