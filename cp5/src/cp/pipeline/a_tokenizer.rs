use std::collections::VecDeque;

pub fn tokenize(chars: &mut VecDeque<char>) -> Option<Token> {
    let mut buf = String::new();

    while let Some(ch) = chars.pop_front() {
        if ch.is_whitespace() {
            break;
        }

        buf.push(ch);

        if buf == "{" {
            return Some(Token::CurlyBracketOpen);
        }
        if buf == "}" {
            return Some(Token::CurlyBracketClose);
        }
    }

    if buf.is_empty() {
        return None;
    }

    Some(Token::Ident(buf))
}

#[derive(Debug)]
pub enum Token {
    CurlyBracketOpen,
    CurlyBracketClose,
    Ident(String),
}
