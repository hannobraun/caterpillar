use std::mem;

pub fn tokenize(source: String) -> Vec<Token> {
    let mut state = State::Initial;
    let mut buffer = Buffer::default();

    let mut tokens = Vec::new();

    for ch in source.chars() {
        match state {
            State::Initial => match ch {
                '#' => {
                    buffer.take_identifier(&mut tokens);
                    state = State::CommentStart;
                }
                ':' => {
                    tokens.push(Token::FunctionName {
                        name: buffer.take(),
                    });
                }
                '{' => {
                    buffer.take_identifier(&mut tokens);
                    tokens.push(Token::CurlyBracketOpen);
                }
                '|' => {
                    buffer.take_identifier(&mut tokens);
                    tokens.push(Token::BranchHeadBoundary);
                }
                ch if ch.is_whitespace() => {
                    buffer.take_identifier(&mut tokens);
                }
                ch => {
                    buffer.push(ch);
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

#[derive(Debug)]
pub enum Token {
    BranchHeadBoundary,
    Comment { text: String },
    CurlyBracketOpen,
    FunctionName { name: String },
    Identifier { name: String },
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

    pub fn take_identifier(&mut self, tokens: &mut Vec<Token>) {
        tokens.extend(
            self.take_if_not_empty()
                .map(|name| Token::Identifier { name }),
        );
    }
}
