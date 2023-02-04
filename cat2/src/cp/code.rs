pub struct Code {
    pub inner: &'static str,
}

impl Code {
    pub fn new() -> Self {
        Self {
            inner: include_str!("../caterpillar/cell_is_born.cp0"),
        }
    }
}
