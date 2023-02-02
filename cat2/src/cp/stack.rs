pub struct Stack {
    pub inner: Vec<Value>,
}

impl Stack {
    pub fn from_values(values: &[Value]) -> Self {
        let inner = Vec::from(values);
        Self { inner }
    }

    pub fn push(&mut self, value: Value) {
        self.inner.push(value)
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.inner.pop()
    }
}

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    U8(u8),
}
