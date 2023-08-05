pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for token in code.split_whitespace() {
        let token = Token::FnRef(token.into());
        tokens.push(token);
    }

    tokens
}

pub enum Token {
    FnRef(String),
}
