use std::iter;

pub struct Tokenizer;

impl Tokenizer {
    pub fn tokenize(
        code: &mut impl Iterator<Item = char>,
    ) -> impl Iterator<Item = String> + '_ {
        iter::from_fn(|| {
            let mut token = String::new();
            token.extend(
                code.by_ref()
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
