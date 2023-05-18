use std::collections::VecDeque;

pub fn tokenize(chars: &mut VecDeque<char>) -> Option<Token> {
    let mut buf = String::new();

    while let Some(ch) = chars.pop_front() {
        if ch.is_whitespace() {
            break;
        }

        buf.push(ch)
    }

    if buf.is_empty() {
        return None;
    }

    Some(Token::Ident(buf))
}

pub enum Token {
    Ident(String),
}
