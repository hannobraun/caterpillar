#[derive(Debug, Default)]
pub struct DataStack {
    values: Vec<usize>,
}

impl DataStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, value: usize) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> usize {
        self.values.pop().unwrap()
    }

    pub fn num_values(&self) -> usize {
        self.values.len()
    }
}
