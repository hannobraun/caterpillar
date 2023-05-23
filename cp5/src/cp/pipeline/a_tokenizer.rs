use std::collections::VecDeque;

pub fn tokenize(chars: &mut VecDeque<char>) -> Option<Token> {
    let mut buf = String::new();

    while let Some(ch) = chars.pop_front() {
        if ch.is_whitespace() && buf.is_empty() {
            continue;
        }
        if ch.is_whitespace() {
            break;
        }

        buf.push(ch);

        match buf.as_str() {
            "{" => {
                return Some(Token::CurlyBracketOpen);
            }
            "}" => {
                return Some(Token::CurlyBracketClose);
            }
            "fn" => {
                return Some(Token::Fn);
            }
            _ => {}
        }
    }

    if buf.is_empty() {
        return None;
    }

    Some(Token::Ident(buf))
}

#[derive(Clone, Debug)]
pub enum Token {
    CurlyBracketOpen,
    CurlyBracketClose,
    Fn,
    Ident(String),
}
