pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for token in code.split_whitespace() {
        let token = tokenize_token(token);
        tokens.push(token);
    }

    tokens
}

fn tokenize_token(token: &str) -> Token {
    match token.split_once(':') {
        Some(("", symbol)) => Token::Symbol(symbol.into()),
        _ => Token::FnRef(token.into()),
    }
}

pub enum Token {
    FnRef(String),
    Symbol(String),
}
