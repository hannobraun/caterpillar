pub struct Stack {
    pub inner: Vec<Value>,
}

impl Stack {
    pub fn pop(&mut self) -> Option<Value> {
        self.inner.pop()
    }
}

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    U8(u8),
}
