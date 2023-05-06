use async_recursion::async_recursion;

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
        match self.tokenizer.peek().await? {
            Token::CurlyBracketOpen => self.parse_block().await,
            Token::Ident(_) => self.parse_word().await,
            token => Err(ParserError::UnexpectedToken(token.clone())),
        }
    }

    async fn parse_word(&mut self) -> Result<SyntaxElement, ParserError> {
        let token = self.tokenizer.next().await?;

        match token {
            Token::Ident(ident) => Ok(SyntaxElement::Word(ident)),
            token => Err(ParserError::UnexpectedToken(token)),
        }
    }

    #[async_recursion(?Send)]
    async fn parse_block(&mut self) -> Result<SyntaxElement, ParserError> {
        let mut syntax_tree = Vec::new();

        let token = self.tokenizer.next().await?;
        if token != Token::CurlyBracketOpen {
            return Err(ParserError::UnexpectedToken(token));
        }

        loop {
            let token = self.tokenizer.peek().await?;

            let syntax_element = match token {
                Token::CurlyBracketClose => {
                    self.tokenizer.next().await?;
                    return Ok(SyntaxElement::Block { syntax_tree });
                }
                _ => self.parse().await?,
            };

            syntax_tree.push(syntax_element);
        }
    }
}

pub enum SyntaxElement {
    Block { syntax_tree: Vec<SyntaxElement> },
    Word(String),
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
