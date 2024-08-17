use std::mem;

pub fn tokenize(source: String) -> Vec<Token> {
    let mut state = State::Initial;
    let mut buffer = Buffer::default();

    let mut tokens = Vec::new();

    for ch in source.chars() {
        match state {
            State::Initial => match ch {
                '#' => {
                    state = State::Comment;
                }
                c => {
                    eprintln!("Unexpected char: `{c}`");
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
}
