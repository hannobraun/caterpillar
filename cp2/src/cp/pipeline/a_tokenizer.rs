use crate::cp::tokens::{Token, Tokens};

pub fn tokenize(code: impl IntoIterator<Item = char>) -> Tokens {
    let code = code.into_iter();
    let mut tokens = Vec::new();

    let mut state = State::Searching;

    for ch in code {
        let next_state = match &mut state {
            State::Searching => {
                if ch.is_whitespace() {
                    continue;
                }

                let buf = String::from(ch);
                State::Processing { buf }
            }
            State::Processing { buf } => {
                if !ch.is_whitespace() {
                    buf.push(ch);
                    continue;
                }

                match_token(buf.as_str(), &mut tokens);

                State::Searching
            }
        };

        state = next_state;
    }

    if let State::Processing { buf } = state {
        match_token(&buf, &mut tokens);
    }

    Tokens(tokens.into())
}

enum State {
    Searching,
    Processing { buf: String },
}

fn match_token(token: &str, tokens: &mut Vec<Token>) {
    let token = Token::match_delimited(token);
    tokens.push(token);
}
