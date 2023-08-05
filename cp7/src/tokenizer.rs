pub fn tokenize(code: &str) -> Vec<Token> {
    code.split_whitespace()
        .map(|token| Token::FnRef(token.into()))
        .collect()
}

pub enum Token {
    FnRef(String),
}
