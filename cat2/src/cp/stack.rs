pub struct Stack {
    pub inner: Vec<Value>,
}

impl Stack {
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
