use std::iter;

pub struct Tokenizer {
    buf: String,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self { buf: String::new() }
    }

    pub async fn tokenize<'r>(
        &'r mut self,
        mut code: impl Iterator<Item = char> + 'r,
    ) -> impl Iterator<Item = String> + 'r {
        iter::from_fn(move || loop {
            let ch = match code.next() {
                Some(ch) => ch,
                None => {
                    if self.buf.is_empty() {
                        return None;
                    }

                    let token = self.buf.clone();
                    self.buf.clear();
                    return Some(token);
                }
            };

            if ch.is_whitespace() {
                let token = self.buf.clone();
                self.buf.clear();
                return Some(token);
            }

            self.buf.push(ch);
        })
    }
}
