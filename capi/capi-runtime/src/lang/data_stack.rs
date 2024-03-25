pub struct DataStack {
    values: Vec<usize>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: usize) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> usize {
        self.values.pop().unwrap()
    }
}
