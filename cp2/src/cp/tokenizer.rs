pub struct Tokens(pub Vec<String>);

pub fn tokenize(code: &str) -> Tokens {
    let tokens = code.split_whitespace().map(Into::into).collect();
    Tokens(tokens)
}
