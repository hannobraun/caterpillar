use std::collections::VecDeque;

use async_trait::async_trait;

use super::pipeline::{a_tokenizer::TokenizerError, b_parser::ParserError};

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    pub elements: VecDeque<SyntaxElement>,
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    Block { syntax_tree: SyntaxTree },
    Function { name: String, body: SyntaxTree },
    Word(String),
}

#[async_trait(?Send)]
pub trait SyntaxSource {
    async fn next(&mut self) -> Result<SyntaxElement, ParserError>;
}

#[async_trait(?Send)]
impl SyntaxSource for SyntaxTree {
    async fn next(&mut self) -> Result<SyntaxElement, ParserError> {
        self.elements
            .pop_front()
            .ok_or(ParserError::Tokenizer(TokenizerError::NoMoreChars))
    }
}
