pub type Tokens = Vec<Token>;

pub enum Token {
    Fn(String),
    BlockOpen,
}

pub fn tokenize(code: &str) -> Tokens {
    code.split_whitespace()
        .map(|token| match token {
            "{" => Token::BlockOpen,
            _ => Token::Fn(token.to_string()),
        })
        .collect()
}
