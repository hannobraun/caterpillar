pub struct DataStack {
    values: Vec<bool>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: bool) {
        self.values.push(value)
    }

    pub fn pop(&mut self) -> Option<bool> {
        self.values.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
