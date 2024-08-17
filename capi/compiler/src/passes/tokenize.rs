pub fn tokenize(source: String) -> Vec<Token> {
    let mut state = State::Initial;
    let mut buffer = Buffer {
        inner: String::new(),
    };

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
                        text: buffer.inner.clone(),
                    });
                    buffer.inner.clear();
                    state = State::Initial;
                }
                c => {
                    buffer.inner.push(c);
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

struct Buffer {
    inner: String,
}
