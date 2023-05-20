pub mod a_tokenizer;
pub mod b_parser;
pub mod d_evaluator;

pub enum PipelineError<T> {
    NotEnoughInput,
    Stage(T),
}
