pub struct Tokens(pub Vec<Token>);

pub enum Token {
    Word(String),
}

pub fn tokenize(code: &str) -> Tokens {
    let tokens = code
        .split_whitespace()
        .map(|token| match token {
            token => Token::Word(token.into()),
        })
        .collect();
    Tokens(tokens)
}
