pub mod a_tokenizer;
pub mod b_parser;
pub mod c_analyzer;
pub mod d_evaluator;

pub use self::{
    a_tokenizer::Tokenizer,
    b_parser::parse,
    c_analyzer::analyze,
    d_evaluator::{evaluate, Error as EvaluatorError},
};

pub struct Pipeline {
    pub tokenizer: Tokenizer,
}
