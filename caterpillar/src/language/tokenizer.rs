use std::iter;

pub struct Tokenizer {
    buf: String,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self { buf: String::new() }
    }

    pub fn tokenize<'r>(
        &'r mut self,
        chars: &'r mut impl Iterator<Item = char>,
    ) -> impl Iterator<Item = String> + 'r {
        iter::from_fn(|| {
            self.buf.extend(
                chars
                    .skip_while(|ch| ch.is_whitespace())
                    .take_while(|ch| !ch.is_whitespace()),
            );

            if self.buf.is_empty() {
                return None;
            }

            let token = self.buf.clone();
            self.buf.clear();

            Some(token)
        })
    }
}
