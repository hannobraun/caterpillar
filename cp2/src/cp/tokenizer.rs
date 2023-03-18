pub struct Tokens(pub Vec<Token>);

#[derive(Debug)]
pub enum Token {
    BindingOperator,
    RoundBracketOpen,
    Word(String),
}

pub fn tokenize(code: &str) -> Tokens {
    let tokens = code
        .split_whitespace()
        .map(|token| match token {
            "=>" => Token::BindingOperator,
            "(" => Token::RoundBracketOpen,
            token => Token::Word(token.into()),
        })
        .collect();
    Tokens(tokens)
}
