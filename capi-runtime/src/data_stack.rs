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

    pub fn clear(&mut self) {
        self.values.clear();
        self.saved.clear();
    }

    pub fn num_values(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn push(&mut self, value: impl Into<Value>) {
        self.values.push(value.into());
    }

    pub fn pop(&mut self) -> Result<Value, StackUnderflow> {
        self.values.pop().ok_or(StackUnderflow)
    }

    pub fn save(&mut self, num: u8) -> Result<(), StackUnderflow> {
        for _ in 0..num {
            let value = self.pop()?;
            self.saved.push(value);
        }

        Ok(())
    }

    pub fn restore(&mut self) {
        while let Some(x) = self.saved.pop() {
            self.push(x);
        }
    }

    pub fn values(&self) -> impl Iterator<Item = Value> + '_ {
        self.values.iter().copied()
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
pub struct Value(pub i8);

impl From<i8> for Value {
    fn from(value: i8) -> Self {
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
pub struct StackUnderflow;
