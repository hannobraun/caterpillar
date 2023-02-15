pub type Tokens = Vec<Token>;

pub enum Token {
    Fn(String),
}

pub fn tokenize(code: &str) -> Tokens {
    code.split_whitespace()
        .map(String::from)
        .map(Token::Fn)
        .collect()
}
