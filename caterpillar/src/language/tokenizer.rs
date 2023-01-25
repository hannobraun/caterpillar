use std::iter;

pub type Buf = String;

pub enum Token {
    /// Refers to a function
    Fn { name: String },

    /// Opens an array
    ArrayOpen,

    /// Closes an array
    ArrayClose,
}

pub fn tokenize<'r>(
    mut chars: impl Iterator<Item = char> + 'r,
    buf: &'r mut Buf,
) -> impl Iterator<Item = Token> + 'r {
    iter::from_fn(move || {
        let mut state = State::NotStarted;

        for ch in &mut chars {
            match state {
                State::NotStarted => {
                    if ch == '[' {
                        return Some(Token::ArrayOpen);
                    }
                    if ch == ']' {
                        return Some(Token::ArrayClose);
                    }

                    if !ch.is_whitespace() {
                        state = State::ReadingFn;
                        buf.push(ch);
                    }
                }
                State::ReadingFn => {
                    if ch.is_whitespace() {
                        break;
                    }

                    buf.push(ch);
                }
            }
        }

        if buf.is_empty() {
            return None;
        }

        let token = Token::Fn { name: buf.clone() };
        buf.clear();

        Some(token)
    })
}

enum State {
    NotStarted,
    ReadingFn,
}
