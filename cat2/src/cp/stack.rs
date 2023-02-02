pub struct Stack {
    pub inner: Vec<Value>,
}

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    U8(u8),
}
