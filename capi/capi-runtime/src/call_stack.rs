use crate::InstructionAddress;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct CallStack {
    pub inner: Vec<InstructionAddress>,
}
