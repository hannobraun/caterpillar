use std::collections::VecDeque;

pub fn tokenize(chars: &mut VecDeque<char>) -> Option<Token> {
    let mut word = String::new();

    while let Some(ch) = chars.pop_front() {
        if ch.is_whitespace() {
            break;
        }

        word.push(ch)
    }

    if word.is_empty() {
        return None;
    }

    Some(Token::Ident(word))
}

pub enum Token {
    Ident(String),
}
