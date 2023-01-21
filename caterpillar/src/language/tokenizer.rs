use std::iter;

pub fn tokenize<'r>(
    chars: &'r mut impl Iterator<Item = char>,
    buf: &'r mut String,
) -> impl Iterator<Item = String> + 'r {
    iter::from_fn(|| {
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
