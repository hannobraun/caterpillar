pub type Tokens = Vec<Token>;

#[derive(Eq, PartialEq)]
pub enum Token {
    Fn(String),
    BlockOpen,
    BlockClose,
}

pub fn tokenize(code: &str) -> Tokens {
    code.split_whitespace()
        .map(|token| match token {
            "{" => Token::BlockOpen,
            "}" => Token::BlockClose,
            _ => Token::Fn(token.to_string()),
        })
        .collect()
}
