pub struct Code {
    pub inner: &'static str,
}

impl Code {
    pub fn new() -> Self {
        let inner = include_str!("../caterpillar/cell_is_born.cp0");
        Self { inner }
    }
}
