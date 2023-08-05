pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for token in code.split_whitespace() {
        let token = tokenize_token(token);
        tokens.push(token);
    }

    tokens
}

fn tokenize_token(token: &str) -> Token {
    match token {
        "{" => return Token::CurlyBracketOpen,
        "}" => return Token::CurlyBracketClose,
        _ => {}
    }

    if let Some(("", symbol)) = token.split_once(':') {
        return Token::Symbol(symbol.into());
    }

    Token::FnRef(token.into())
}

pub enum Token {
    CurlyBracketOpen,
    CurlyBracketClose,
    FnRef(String),
    Symbol(String),
}
