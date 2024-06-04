use crate::{runtime::Function, InstructionAddress};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallStack {
    inner: Vec<InstructionAddress>,
}

impl CallStack {
    pub fn new(next: Function) -> Self {
        let next = next.iter().copied().next().unwrap();
        Self { inner: vec![next] }
    }

    pub fn next(&self) -> Option<InstructionAddress> {
        self.inner.last().copied()
    }

    pub fn contains(&self, address: InstructionAddress) -> bool {
        self.inner
            .iter()
            .any(|stack_address| stack_address == &address.next())
    }

    pub fn advance(&mut self) -> Option<InstructionAddress> {
        let address = self.next();
        self.inner.last_mut().unwrap().increment();
        address
    }

    pub fn push(
        &mut self,
        function: Function,
    ) -> Result<(), CallStackOverflow> {
        let address = function.iter().copied().next().unwrap();
        self.inner.push(address);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<InstructionAddress> {
        self.inner.pop()
    }

    pub fn iter(&self) -> impl Iterator<Item = &InstructionAddress> {
        self.inner.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallStackOverflow;
