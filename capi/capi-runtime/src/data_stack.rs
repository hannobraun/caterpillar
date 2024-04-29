use core::fmt;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct DataStack {
    values: Vec<Value>,
    saved: Vec<Value>,
}

impl DataStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
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

    pub fn save(&mut self, num: u32) {
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
pub struct Value(pub u32);

impl From<u32> for Value {
    fn from(value: u32) -> Self {
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
