use std::mem;

pub fn tokenize(source: String) -> Vec<Token> {
    let eager_tokens = vec![
        ("{", Token::FunctionStart),
        ("}", Token::FunctionEnd),
        ("|", Token::BranchHeadBoundary),
        ("=>", Token::BindingStart),
        (".", Token::BindingEnd),
    ];

    let mut state = State::Initial;
    let mut buffer = Buffer::default();

    let mut tokens = Vec::new();

    for ch in source.chars() {
        match state {
            State::Initial => match ch {
                '#' => {
                    buffer.take_literal_or_identifier(&mut tokens);
                    state = State::CommentStart;
                }
                ':' => {
                    tokens.push(Token::FunctionName {
                        name: buffer.take(),
                    });
                }
                ch if ch.is_whitespace() => {
                    buffer.take_literal_or_identifier(&mut tokens);
                }
                ch => {
                    buffer.push(ch);

                    for (s, token) in &eager_tokens {
                        if buffer.take_from_end(s) {
                            buffer.take_literal_or_identifier(&mut tokens);
                            tokens.push(token.clone());
                        }
                    }
                }
            },

            State::CommentStart | State::CommentText if ch == '\n' => {
                tokens.push(Token::Comment {
                    text: buffer.take(),
                });
                state = State::Initial;
            }
            State::CommentStart => match ch {
                ch if ch.is_whitespace() => {}
                ch => {
                    buffer.push(ch);
                    state = State::CommentText;
                }
            },
            State::CommentText => {
                buffer.push(ch);
                state = State::CommentText
            }
        }
    }

    tokens
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Comment { text: String },

    FunctionName { name: String },
    FunctionStart,
    FunctionEnd,
    BranchHeadBoundary,

    Identifier { name: String },
    IntegerLiteral { value: i32 },

    BindingStart,
    BindingEnd,
}

enum State {
    Initial,
    CommentStart,
    CommentText,
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

    pub fn take_literal_or_identifier(&mut self, tokens: &mut Vec<Token>) {
        tokens.extend(self.take_if_not_empty().map(|token| {
            if let Ok(value) = token.parse() {
                Token::IntegerLiteral { value }
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
