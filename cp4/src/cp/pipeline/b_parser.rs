use super::a_tokenizer::{Token, Tokenizer};

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    pub async fn next_token(&mut self) -> Option<SyntaxElement> {
        let Some(token) = self.tokenizer.next_token().await else {
            return None;
        };

        match token {
            Token::Ident(ident) => Some(SyntaxElement::Word(ident)),
        }
    }
}

pub enum SyntaxElement {
    Word(String),
}
