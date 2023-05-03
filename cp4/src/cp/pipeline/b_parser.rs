use super::a_tokenizer::{Token, Tokenizer};

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }

    pub async fn next_token(&mut self) -> Option<SyntaxElement> {
        match self.tokenizer.next_token().await? {
            Token::Ident(ident) => Some(SyntaxElement::Word(ident)),
        }
    }
}

pub enum SyntaxElement {
    Word(String),
}
