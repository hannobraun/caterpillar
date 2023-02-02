pub struct Stack {
    inner: Vec<Value>,
}

impl Stack {
    pub fn from_values(values: &[Value]) -> Self {
        let inner = Vec::from(values);
        Self { inner }
    }

    pub fn pop_any(&mut self) -> Value {
        self.inner.pop().expect("Stack is empty")
    }

    pub fn pop_bool(&mut self) -> bool {
        let Value::Bool(value) = self.pop_any() else {
            panic!("Expected `bool`")
        };
        value
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
