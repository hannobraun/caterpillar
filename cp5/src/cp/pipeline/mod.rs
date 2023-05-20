pub mod a_tokenizer;
pub mod b_parser;
pub mod d_evaluator;

pub mod stage_input;

#[derive(Debug, thiserror::Error)]
pub enum PipelineError<T> {
    #[error(transparent)]
    NotEnoughInput(#[from] stage_input::NoMoreInput),
    Stage(T),
}
