use crate::{runtime::Function, InstructionAddress};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallStack {
    inner: Vec<Function>,
}

impl CallStack {
    pub fn new(next: Function) -> Self {
        Self { inner: vec![next] }
    }

    pub fn next(&self) -> Option<InstructionAddress> {
        self.inner
            .last()
            .and_then(|function| function.front().copied())
    }

    pub fn contains(&self, address: InstructionAddress) -> bool {
        self.inner
            .iter()
            .any(|function| function.front() == Some(&address.next()))
    }

    pub fn advance(&mut self) -> Option<InstructionAddress> {
        let function = self.inner.last_mut()?;
        function.pop_front()
    }

    pub fn push(
        &mut self,
        function: Function,
    ) -> Result<(), CallStackOverflow> {
        self.inner.push(function);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Function> {
        self.inner.pop()
    }

    pub fn iter(&self) -> impl Iterator<Item = &InstructionAddress> {
        self.inner.iter().filter_map(|function| function.front())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallStackOverflow;
