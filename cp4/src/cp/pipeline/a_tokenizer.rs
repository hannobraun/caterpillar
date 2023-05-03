use std::pin::Pin;

use futures::{Stream, StreamExt};

pub struct Tokenizer {
    chars: Chars,
    buf: String,
}

impl Tokenizer {
    pub fn new(chars: Chars) -> Self {
        Self {
            chars,
            buf: String::new(),
        }
    }

    pub async fn next_token(&mut self) -> Option<Token> {
        loop {
            let ch = match self.chars.next().await {
                Some(ch) => ch,
                None => {
                    if self.buf.is_empty() {
                        return None;
                    }

                    return Some(Token::ident(&mut self.buf));
                }
            };

            if ch.is_whitespace() {
                return Some(Token::ident(&mut self.buf));
            }

            self.buf.push(ch);
        }
    }
}

pub type Chars = Pin<Box<dyn Stream<Item = char>>>;

pub enum Token {
    Ident(String),
}

impl Token {
    fn ident(buf: &mut String) -> Self {
        let token = buf.clone();
        buf.clear();
        Self::Ident(token)
    }
}
