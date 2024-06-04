use crate::InstructionAddress;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct CallStack {
    inner: Vec<InstructionAddress>,
}

impl CallStack {
    pub fn contains(&self, address: InstructionAddress) -> bool {
        self.inner.contains(&address.next())
    }

    pub fn push(
        &mut self,
        address: InstructionAddress,
    ) -> Result<(), CallStackOverflow> {
        self.inner.push(address);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<InstructionAddress> {
        self.inner.pop()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &InstructionAddress> {
        self.inner.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallStackOverflow;
