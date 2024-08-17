pub fn tokenize(_source: String) -> Vec<Token> {
    let mut state = State::Initial;
    let mut buffer = String::new();

    let mut tokens = Vec::new();

    for c in _source.chars() {
        match state {
            State::Initial => match c {
                '#' => {
                    state = State::Comment;
                }
                c => {
                    eprintln!("Unexpected char: `{c}`");
                }
            },
            State::Comment => match c {
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
