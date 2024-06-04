use crate::{runtime::Function, InstructionAddress};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CallStack {
    frames: Vec<Function>,
}

impl CallStack {
    pub fn new(next: Function) -> Self {
        let mut self_ = Self { frames: Vec::new() };
        self_
            .push(next)
            .expect("Expected recursion limit to be more than zero.");
        self_
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

    pub fn push(
        &mut self,
        function: Function,
    ) -> Result<(), CallStackOverflow> {
        if self.frames.len() >= RECURSION_LIMIT {
            return Err(CallStackOverflow);
        }

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

const RECURSION_LIMIT: usize = 8;
