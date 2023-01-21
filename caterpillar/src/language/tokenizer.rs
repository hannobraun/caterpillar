use std::iter;

pub fn tokenize(
    chars: &mut impl Iterator<Item = char>,
) -> impl Iterator<Item = String> + '_ {
    iter::from_fn(|| {
        let mut buf = String::new();

        buf.extend(
            chars
                .skip_while(|ch| ch.is_whitespace())
                .take_while(|ch| !ch.is_whitespace()),
        );

        if buf.is_empty() {
            return None;
        }

        let token = buf.clone();
        buf.clear();

        Some(token)
    })
}
