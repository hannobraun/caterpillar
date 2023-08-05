pub struct DataStack {
    values: Vec<Value>,
}

impl DataStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value)
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.values.pop()
    }
}

pub enum Value {
    Number(Number),
}

pub type Number = i64;
