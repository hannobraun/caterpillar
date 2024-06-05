use crate::cp::tokens::{Token, Tokens};

pub fn tokenize(code: impl IntoIterator<Item = char>) -> Tokens {
    let code = code.into_iter();
    let mut tokens = Vec::new();

    let mut state = State::Searching;

    for ch in code {
        match &mut state {
            State::Searching => {
                if ch.is_whitespace() {
                    continue;
                }

                let buf = String::from(ch);
                state = State::Processing { buf }
            }
            State::Processing { buf } => {
                if Token::match_eagerly(buf, &mut tokens) {
                    buf.clear();
                }

                if !ch.is_whitespace() {
                    buf.push(ch);
                    continue;
                }

                if !buf.is_empty() {
                    Token::match_delimited(buf.as_str(), &mut tokens);
                    state = State::Searching;
                }
            }
        }
    }

    if let State::Processing { buf } = state {
        Token::match_delimited(&buf, &mut tokens);
    }

    Tokens(tokens.into())
}

enum State {
    Searching,
    Processing { buf: String },
}
