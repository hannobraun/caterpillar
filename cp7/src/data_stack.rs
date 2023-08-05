pub struct DataStack {
    values: Vec<i64>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: i64) {
        self.values.push(value)
    }

    pub fn pop(&mut self) -> Option<i64> {
        self.values.pop()
    }
}
