use crate::language::pipeline::concepts::tokens::{Token, Tokens};

pub fn tokenize(code: &str) -> Tokens {
    let mut chars = code.chars();

    let mut tokens = Vec::new();
    let mut state = State::Scanning;

    loop {
        let Some(ch) = chars.next() else { break };

        match state {
            State::Scanning => match ch {
                ch if ch.is_whitespace() => {
                    continue;
                }
                '{' => {
                    tokens.push(Token::CurlyBracketOpen);
                }
                '}' => {
                    tokens.push(Token::CurlyBracketClose);
                }
                '#' => {
                    state = State::Comment;
                }
                ':' => {
                    state = State::Symbol { buf: String::new() };
                }
                ch => {
                    state = State::WordOrNumber {
                        buf: String::from(ch),
                    };
                }
            },
            State::Comment => {
                if ch == '\n' {
                    state = State::Scanning;
                }
            }
            State::Symbol { mut buf } => {
                if ch.is_whitespace() {
                    tokens.push(Token::Symbol(buf));
                    state = State::Scanning;
                    continue;
                }

                buf.push(ch);
                state = State::Symbol { buf };
            }
            State::WordOrNumber { mut buf } => {
                if ch.is_whitespace() {
                    let token = match buf.parse::<i64>() {
                        Ok(number) => Token::Number(number),
                        Err(_) => Token::Word(buf),
                    };

                    tokens.push(token);

                    state = State::Scanning;
                    continue;
                }

                buf.push(ch);
                state = State::WordOrNumber { buf };
            }
        }
    }

    Tokens::from(tokens)
}

enum State {
    Scanning,
    Comment,
    Symbol { buf: String },
    WordOrNumber { buf: String },
}
