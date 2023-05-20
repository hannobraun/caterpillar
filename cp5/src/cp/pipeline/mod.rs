use std::collections::VecDeque;

pub mod a_tokenizer;
pub mod b_parser;
pub mod d_evaluator;

#[derive(Debug)]
pub struct StageInput<T> {
    pub elements: VecDeque<T>,
}

pub enum PipelineError<T> {
    NotEnoughInput,
    Stage(T),
}
