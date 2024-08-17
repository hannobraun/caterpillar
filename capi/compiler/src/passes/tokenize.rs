pub fn tokenize(_source: String) -> Vec<Token> {
    let mut state = State::Initial;
    let mut tokens = Vec::new();

    for c in _source.chars() {
        match state {
            State::Initial => match c {
                '#' => {
                    state = State::Comment {
                        buffer: String::new(),
                    };
                }
                c => {
                    eprintln!("Unexpected char: `{c}`");
                }
            },
            State::Comment { mut buffer } => match c {
                '\n' => {
                    tokens.push(Token::Comment { text: buffer });
                    state = State::Initial;
                }
                c => {
                    buffer.push(c);
                    state = State::Comment { buffer }
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
    Comment { buffer: String },
}
