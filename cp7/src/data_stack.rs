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

impl From<Number> for Value {
    fn from(number: Number) -> Self {
        Self::Number(number)
    }
}

pub type Number = i64;
