use crate::cp::tokens::{Token, Tokens};

pub fn tokenize(code: &str) -> Tokens {
    let code = code.chars();
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
    let token = match token {
        "fn" => Token::Function,
        "test" => Token::Test,
        "=>" => Token::BindingOperator,
        "." => Token::Period,
        "{" => Token::CurlyBracketOpen,
        "}" => Token::CurlyBracketClose,
        "(" => Token::RoundBracketOpen,
        ")" => Token::RoundBracketClose,
        "[" => Token::SquareBracketOpen,
        "]" => Token::SquareBracketClose,
        token => {
            if let Some(("", symbol)) = token.split_once(':') {
                Token::Symbol(symbol.into())
            } else {
                Token::Ident(token.into())
            }
        }
    };

    tokens.push(token);
}
