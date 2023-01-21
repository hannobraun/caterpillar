use std::iter;

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize<'r>(
        &mut self,
        chars: &'r mut impl Iterator<Item = char>,
    ) -> impl Iterator<Item = String> + 'r {
        iter::from_fn(|| {
            let mut token = String::new();
            token.extend(
                chars
                    .skip_while(|ch| ch.is_whitespace())
                    .take_while(|ch| !ch.is_whitespace()),
            );

            if token.is_empty() {
                return None;
            }

            Some(token)
        })
    }
}
