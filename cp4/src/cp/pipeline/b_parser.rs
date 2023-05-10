use std::collections::VecDeque;

use async_recursion::async_recursion;
use async_trait::async_trait;

use crate::cp::syntax::{SyntaxElement, SyntaxSource, SyntaxTree};

use super::a_tokenizer::{Token, Tokenizer, TokenizerError};

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    pub async fn next(&mut self) -> Result<SyntaxElement, ParserError> {
        self.parse().await
    }

    #[async_recursion(?Send)]
    async fn parse(&mut self) -> Result<SyntaxElement, ParserError> {
        let syntax_element = match self.tokenizer.peek().await? {
            Token::CurlyBracketOpen => {
                let syntax_tree = self.parse_block().await?;
                SyntaxElement::Block { syntax_tree }
            }
            Token::Fn => self.parse_fn().await?,
            Token::Ident(_) => self.parse_word().await?,
            token => return Err(ParserError::UnexpectedToken(token.clone())),
        };

        Ok(syntax_element)
    }

    #[async_recursion(?Send)]
    async fn parse_block(&mut self) -> Result<SyntaxTree, ParserError> {
        let mut syntax_tree = SyntaxTree {
            elements: VecDeque::new(),
        };

        self.expect(Token::CurlyBracketOpen).await?;

        loop {
            let token = self.tokenizer.peek().await?;

            let syntax_element = match token {
                Token::CurlyBracketClose => {
                    self.tokenizer.next().await?;
                    return Ok(syntax_tree);
                }
                _ => self.parse().await?,
            };

            syntax_tree.elements.push_back(syntax_element);
        }
    }

    async fn parse_fn(&mut self) -> Result<SyntaxElement, ParserError> {
        self.expect(Token::Fn).await?;

        let token = self.tokenizer.next().await?;
        let Token::Ident(name) = token else {
            return Err(ParserError::UnexpectedToken(token));
        };

        let body = self.parse_block().await?;

        Ok(SyntaxElement::Function { name, body })
    }

    async fn parse_word(&mut self) -> Result<SyntaxElement, ParserError> {
        match self.tokenizer.next().await? {
            Token::Ident(ident) => Ok(SyntaxElement::Word(ident)),
            token => Err(ParserError::UnexpectedToken(token)),
        }
    }

    async fn expect(&mut self, expected: Token) -> Result<(), ParserError> {
        let token = self.tokenizer.next().await?;

        if token != expected {
            return Err(ParserError::UnexpectedToken(token));
        }

        Ok(())
    }
}

#[async_trait(?Send)]
impl SyntaxSource for Parser {
    async fn next(&mut self) -> Result<SyntaxElement, ParserError> {
        self.next().await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error(transparent)]
    Tokenizer(#[from] TokenizerError),

    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
}

impl ParserError {
    pub fn is_no_more_chars(&self) -> bool {
        if let Self::Tokenizer(TokenizerError::NoMoreChars) = self {
            return true;
        }

        false
    }
}
