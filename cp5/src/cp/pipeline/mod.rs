use std::collections::VecDeque;

pub mod a_tokenizer;
pub mod b_parser;
pub mod d_evaluator;

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
}

pub enum PipelineError<T> {
    NotEnoughInput,
    Stage(T),
}
