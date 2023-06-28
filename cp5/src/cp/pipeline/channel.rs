use std::collections::VecDeque;

use super::ir::analyzer_output::{AnalyzerEvent, AnalyzerOutput};

#[derive(Debug)]
pub struct PipelineChannel<T> {
    items: VecDeque<T>,
}

impl<T> PipelineChannel<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add(&mut self, item: T) {
        self.items.push_back(item)
    }

    pub fn as_input(&mut self) -> StageInput<T> {
        StageInput {
            channel: self,
            num_read: 0,
        }
    }
}

impl<T> FromIterator<T> for PipelineChannel<T> {
    fn from_iter<I: IntoIterator<Item = T>>(items: I) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

impl From<AnalyzerOutput> for PipelineChannel<AnalyzerEvent> {
    fn from(analyzer_output: AnalyzerOutput) -> Self {
        Self {
            items: analyzer_output.events.into(),
        }
    }
}

#[derive(Debug)]
pub struct StageInput<'r, T> {
    channel: &'r mut PipelineChannel<T>,
    num_read: usize,
}

impl<'r, T> StageInput<'r, T> {
    pub fn peek(&self) -> Result<&T, NoMoreInput> {
        self.channel.items.get(self.num_read).ok_or(NoMoreInput)
    }

    pub fn read(&mut self) -> Result<&T, NoMoreInput> {
        let element =
            self.channel.items.get(self.num_read).ok_or(NoMoreInput)?;
        self.num_read += 1;
        Ok(element)
    }

    pub fn unread_last_n(&mut self, n: usize) {
        self.num_read -= n;
    }

    pub fn take(&mut self) {
        let _ = self.channel.items.drain(..self.num_read).last();
        self.num_read = 0;
    }
}

#[derive(Debug, thiserror::Error)]
#[error("No more input")]
pub struct NoMoreInput;
