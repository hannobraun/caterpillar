use std::collections::VecDeque;

use super::ir::analyzer_output::{AnalyzerEvent, AnalyzerOutput};

#[derive(Debug)]
pub struct StageInput<T> {
    items: VecDeque<T>,
}

impl<T> StageInput<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add(&mut self, element: T) {
        self.items.push_back(element)
    }

    pub fn reader(&mut self) -> StageInputReader<T> {
        StageInputReader {
            inner: self,
            num_read: 0,
        }
    }
}

impl<T> FromIterator<T> for StageInput<T> {
    fn from_iter<I: IntoIterator<Item = T>>(items: I) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

impl From<AnalyzerOutput> for StageInput<AnalyzerEvent> {
    fn from(expressions: AnalyzerOutput) -> Self {
        Self {
            items: expressions.events.into(),
        }
    }
}

#[derive(Debug)]
pub struct StageInputReader<'r, T> {
    inner: &'r mut StageInput<T>,
    num_read: usize,
}

impl<'r, T> StageInputReader<'r, T> {
    pub fn peek(&self) -> Result<&T, NoMoreInput> {
        self.inner.items.get(self.num_read).ok_or(NoMoreInput)
    }

    pub fn read(&mut self) -> Result<&T, NoMoreInput> {
        let element = self.inner.items.get(self.num_read).ok_or(NoMoreInput)?;
        self.num_read += 1;
        Ok(element)
    }

    pub fn unread_last_n(&mut self, n: usize) {
        self.num_read -= n;
    }

    pub fn take(&mut self) {
        let _ = self.inner.items.drain(..self.num_read).last();
        self.num_read = 0;
    }
}

#[derive(Debug, thiserror::Error)]
#[error("No more input")]
pub struct NoMoreInput;
