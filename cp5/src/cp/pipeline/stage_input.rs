use std::collections::VecDeque;

#[derive(Debug)]
pub struct StageInput<T> {
    elements: VecDeque<T>,
}

impl<T> StageInput<T> {
    pub fn new() -> Self {
        Self {
            elements: VecDeque::new(),
        }
    }

    pub fn add(&mut self, element: T) {
        self.elements.push_back(element)
    }

    pub fn reader(&mut self) -> StageInputReader<T> {
        StageInputReader { inner: self }
    }
}

pub struct StageInputReader<'r, T> {
    inner: &'r mut StageInput<T>,
}

impl<'r, T> StageInputReader<'r, T> {
    pub fn peek(&self) -> Result<&T, NoMoreInput> {
        self.inner.elements.front().ok_or(NoMoreInput)
    }

    pub fn next(&mut self) -> Result<T, NoMoreInput> {
        self.inner.elements.pop_front().ok_or(NoMoreInput)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("No more input")]
pub struct NoMoreInput;
