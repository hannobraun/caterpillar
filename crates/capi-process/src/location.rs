use crate::instructions::InstructionIndex;

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct Location {
    pub function: String,
    pub index: InstructionIndex,
}

impl Location {
    pub fn next(mut self) -> Self {
        self.index.increment();
        self
    }
}
