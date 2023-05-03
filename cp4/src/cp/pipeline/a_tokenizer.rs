use std::iter;

pub struct Tokenizer {
    buf: String,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self { buf: String::new() }
    }

    pub fn tokenize(
        &mut self,
        code: impl IntoIterator<Item = char>,
    ) -> impl Iterator<Item = String> {
        let mut code = code.into_iter();
        let mut tokenizer = Tokenizer::new();

        iter::from_fn(move || loop {
            let ch = match code.next() {
                Some(ch) => ch,
                None => {
                    if tokenizer.buf.is_empty() {
                        return None;
                    }

                    let token = tokenizer.buf.clone();
                    tokenizer.buf.clear();
                    return Some(token);
                }
            };

            if ch.is_whitespace() {
                let token = tokenizer.buf.clone();
                tokenizer.buf.clear();
                return Some(token);
            }

            tokenizer.buf.push(ch);
        })
    }
}
