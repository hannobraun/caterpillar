use crate::InstructionAddress;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct CallStack {
    pub inner: CallStackInner,
}

impl CallStack {
    pub fn contains(&self, address: InstructionAddress) -> bool {
        self.inner.contains(&address.next())
    }

    pub fn push(&mut self, address: InstructionAddress) {
        self.inner.push(address);
    }

    pub fn pop(&mut self) -> Option<InstructionAddress> {
        self.inner.pop()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

type CallStackInner = Vec<InstructionAddress>;
