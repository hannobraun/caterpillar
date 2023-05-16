use crate::cp::chars::Chars;

pub struct Tokenizer {
    chars: Chars,
    buf: String,
    next: Option<Token>,
}

impl Tokenizer {
    pub fn new(chars: Chars) -> Self {
        Self {
            chars,
            buf: String::new(),
            next: None,
        }
    }

    pub async fn next(&mut self) -> Result<Token, TokenizerError> {
        if let Some(token) = self.next.take() {
            return Ok(token);
        }

        loop {
            let ch = match self.chars.next().await {
                Some(ch) => ch,
                None => {
                    if self.buf.is_empty() {
                        return Err(TokenizerError::NoMoreChars);
                    }

                    return Ok(Token::from_buf(&mut self.buf));
                }
            };

            if ch.is_whitespace() {
                return Ok(Token::from_buf(&mut self.buf));
            }

            self.buf.push(ch);
        }
    }

    pub async fn peek(&mut self) -> Result<&Token, TokenizerError> {
        let token = self.next().await?;
        self.next = Some(token);

        // Can't panic. We just put a token in there.
        let token = self.next.as_ref().unwrap();

        Ok(token)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    CurlyBracketOpen,
    CurlyBracketClose,
    Fn,
    Ident(String),
}

impl Token {
    fn from_buf(buf: &mut String) -> Self {
        let token = match buf.as_str() {
            "{" => Self::CurlyBracketOpen,
            "}" => Self::CurlyBracketClose,
            "fn" => Self::Fn,
            _ => Self::Ident(buf.clone()),
        };

        buf.clear();

        token
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TokenizerError {
    #[error("No more characters")]
    NoMoreChars,
}
