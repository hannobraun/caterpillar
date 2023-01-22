use std::iter;

pub type Buf = String;

pub enum Token {
    /// A token that refers to a function
    Fn { name: String },
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
                    if !ch.is_whitespace() {
                        state = State::ReadingFn;
                        buf.push(ch);
                    }
                }
                State::ReadingFn => {
                    if ch.is_whitespace() {
                        break;
                    } else {
                        buf.push(ch);
                    }
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
