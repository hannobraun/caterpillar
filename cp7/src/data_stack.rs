pub struct DataStack {
    values: Vec<Number>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: Number) {
        self.values.push(value)
    }

    pub fn pop(&mut self) -> Option<Number> {
        self.values.pop()
    }
}

pub type Number = i64;
