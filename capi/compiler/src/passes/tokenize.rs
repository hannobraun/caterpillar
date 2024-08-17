use std::mem;

pub fn tokenize(source: String) -> Vec<Token> {
    let mut state = State::Initial;
    let mut buffer = Buffer::default();

    let mut tokens = Vec::new();

    for ch in source.chars() {
        match state {
            State::Initial => match ch {
                '#' => {
                    tokens
                        .extend(buffer.take_if_not_empty().map(Token::Invalid));
                    state = State::CommentStart;
                }
                ch => {
                    buffer.push(ch);
                }
            },
            State::CommentStart => match ch {
                '\n' => {
                    tokens.push(Token::Comment {
                        text: buffer.take(),
                    });
                    state = State::Initial;
                }
                ch if ch.is_whitespace() => {}
                ch => {
                    buffer.push(ch);
                    state = State::Comment;
                }
            },
            State::Comment => match ch {
                '\n' => {
                    tokens.push(Token::Comment {
                        text: buffer.take(),
                    });
                    state = State::Initial;
                }
                c => {
                    buffer.push(c);
                    state = State::Comment
                }
            },
        }
    }

    tokens
}

#[derive(Debug)]
pub enum Token {
    Comment { text: String },
    Invalid(String),
}

enum State {
    Initial,
    CommentStart,
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
}
