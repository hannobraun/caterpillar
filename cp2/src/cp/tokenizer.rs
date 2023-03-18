pub struct Tokens(pub Vec<Token>);

pub enum Token {
    /// A word refers to a function or variable
    Word(String),
}

pub fn tokenize(code: &str) -> Tokens {
    let tokens = code
        .split_whitespace()
        .map(Into::into)
        .map(Token::Word)
        .collect();
    Tokens(tokens)
}
