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

    pub fn pop(&mut self) -> Result<bool, PopFromEmptyStack> {
        self.values.pop().ok_or(PopFromEmptyStack)
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Tried to pop value from empty stack")]
pub struct PopFromEmptyStack;
