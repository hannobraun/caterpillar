use std::iter;

pub fn tokenize(
    code: impl IntoIterator<Item = char>,
) -> impl Iterator<Item = String> {
    let mut code = code.into_iter();
    let mut buf = String::new();

    iter::from_fn(move || loop {
        let ch = match code.next() {
            Some(ch) => ch,
            None => {
                if buf.is_empty() {
                    return None;
                }

                let token = buf.clone();
                buf.clear();
                return Some(token);
            }
        };

        if ch.is_whitespace() {
            let token = buf.clone();
            buf.clear();
            return Some(token);
        }

        buf.push(ch);
    })
}
