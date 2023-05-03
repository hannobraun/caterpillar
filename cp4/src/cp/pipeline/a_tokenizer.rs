use std::iter;

pub struct Tokenizer {
    buf: String,
}

pub fn tokenize(
    code: impl IntoIterator<Item = char>,
) -> impl Iterator<Item = String> {
    let mut code = code.into_iter();
    let mut tokenizer = Tokenizer { buf: String::new() };

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
