use crate::{runtime::Function, InstructionAddress};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallStack {
    frames: Vec<Function>,
}

impl CallStack {
    pub fn new(next: Function) -> Self {
        Self { frames: vec![next] }
    }

    pub fn next(&self) -> Option<InstructionAddress> {
        self.frames
            .last()
            .and_then(|function| function.front().copied())
    }

    pub fn contains(&self, address: InstructionAddress) -> bool {
        self.frames
            .iter()
            .any(|function| function.front() == Some(&address.next()))
    }

    pub fn advance(&mut self) -> Option<InstructionAddress> {
        let function = self.frames.last_mut()?;
        function.pop_front()
    }

    pub fn push(
        &mut self,
        function: Function,
    ) -> Result<(), CallStackOverflow> {
        self.frames.push(function);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Function> {
        self.frames.pop()
    }

    pub fn iter(&self) -> impl Iterator<Item = &InstructionAddress> {
        self.frames.iter().filter_map(|function| function.front())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallStackOverflow;
