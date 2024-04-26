use core::fmt;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct DataStack {
    values: Vec<Value>,
    saved: Vec<Value>,
}

impl DataStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clone(&mut self) -> Value {
        self.values.last().copied().unwrap()
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    pub fn pop(&mut self) -> Result<Value, PopFromEmptyStack> {
        self.values.pop().ok_or(PopFromEmptyStack)
    }

    pub fn num_values(&self) -> usize {
        self.values.len()
    }

    pub fn save(&mut self, num: usize) {
        for _ in 0..num {
            let value = self.pop().unwrap();
            self.saved.push(value);
        }
    }

    pub fn restore(&mut self) {
        while let Some(x) = self.saved.pop() {
            self.push(x);
        }
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct Value(pub usize);

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    thiserror::Error,
)]
#[error("Tried to pop value from empty stack")]
pub struct PopFromEmptyStack;
