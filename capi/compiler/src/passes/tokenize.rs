use std::mem;

pub fn tokenize(source: &str) -> Vec<Token> {
    let eager_tokens = vec![
        (r",", Token::Delimiter),
        (r"\", Token::BranchStart),
        (r"->", Token::BranchBodyStart),
    ];

    let mut state = State::Initial;
    let mut buffer = Buffer::default();

    let mut tokens = Vec::new();

    for ch in source.chars() {
        match state {
            State::Initial => match ch {
                '#' => {
                    buffer.take_literal_or_keyword_identifier(&mut tokens);
                    state = State::Comment;
                }
                ':' => {
                    tokens.push(Token::FunctionName {
                        name: buffer.take(),
                    });
                }
                ch if ch.is_whitespace() => {
                    buffer.take_literal_or_keyword_identifier(&mut tokens);
                }
                ch => {
                    buffer.push(ch);

                    for (s, token) in &eager_tokens {
                        if buffer.take_from_end(s) {
                            buffer.take_literal_or_keyword_identifier(
                                &mut tokens,
                            );
                            tokens.push(token.clone());
                        }
                    }
                }
            },
            State::Comment => match ch {
                '\n' => {
                    tokens.push(Token::Comment {
                        text: buffer.take(),
                    });
                    state = State::Initial;
                }
                ch => {
                    buffer.push(ch);
                    state = State::Comment
                }
            },
        }
    }

    tokens
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Comment { text: String },
    Delimiter,

    KeywordEnd,
    KeywordFn,

    FunctionName { name: String },

    BranchStart,
    BranchBodyStart,

    Identifier { name: String },
    IntegerLiteral { value: i32 },
}

enum State {
    Initial,
    Comment,
}

#[derive(Default)]
struct Buffer {
    inner: String,
}

impl Buffer {
    pub fn push(&mut self, ch: char) {
        self.inner.push(ch);
    }

    pub fn take(&mut self) -> String {
        mem::take(&mut self.inner)
    }

    pub fn take_if_not_empty(&mut self) -> Option<String> {
        if self.inner.is_empty() {
            None
        } else {
            Some(self.take())
        }
    }

    pub fn take_literal_or_keyword_identifier(
        &mut self,
        tokens: &mut Vec<Token>,
    ) {
        tokens.extend(self.take_if_not_empty().map(|token| {
            if let Ok(value) = token.parse() {
                Token::IntegerLiteral { value }
            } else if token == "end" {
                Token::KeywordEnd
            } else if token == "fn" {
                Token::KeywordFn
            } else {
                Token::Identifier { name: token }
            }
        }));
    }

    pub fn take_from_end(&mut self, s: &str) -> bool {
        if self.inner.ends_with(s) {
            self.inner.truncate(self.inner.len() - s.len());
            true
        } else {
            false
        }
    }
}
