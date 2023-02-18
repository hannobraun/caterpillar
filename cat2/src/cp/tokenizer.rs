pub type Tokens = Vec<Token>;

#[derive(Clone, Eq, PartialEq)]
pub enum Token {
    Fn(String),
    BlockOpen,
    BlockClose,
    ListOpen,
}

pub fn tokenize(code: &str) -> Tokens {
    code.split_whitespace()
        .map(|token| match token {
            "{" => Token::BlockOpen,
            "}" => Token::BlockClose,
            "[" => Token::ListOpen,
            _ => Token::Fn(token.to_string()),
        })
        .collect()
}
