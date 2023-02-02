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

    pub fn pop_u8(&mut self) -> u8 {
        let Value::U8(value) = self.pop_any() else {
            panic!("Expected `u8`")
        };
        value
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.inner.push(value.into())
    }
}

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    U8(u8),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}
