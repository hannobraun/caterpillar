pub fn tokenize(_source: String) -> Vec<Token> {
    let mut state = State::Initial;
    let mut buffer = String::new();

    let mut tokens = Vec::new();

    for ch in _source.chars() {
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
                        text: buffer.clone(),
                    });
                    buffer.clear();
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
