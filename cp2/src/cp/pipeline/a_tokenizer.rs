use crate::cp::tokens::{Token, Tokens};

pub fn tokenize(code: &str) -> Tokens {
    let code = code.split_whitespace();
    let mut tokens = Vec::new();

    for token in code {
        match_token(token, &mut tokens);
    }

    Tokens(tokens.into())
}

fn match_token(token: &str, tokens: &mut Vec<Token>) {
    let token = match token {
        "fn" => Token::Function,
        "test" => Token::Test,
        "=>" => Token::BindingOperator,
        "." => Token::Period,
        "{" => Token::CurlyBracketOpen,
        "}" => Token::CurlyBracketClose,
        "(" => Token::RoundBracketOpen,
        ")" => Token::RoundBracketClose,
        "[" => Token::SquareBracketOpen,
        "]" => Token::SquareBracketClose,
        token => {
            if let Some(("", symbol)) = token.split_once(':') {
                Token::Symbol(symbol.into())
            } else {
                Token::Ident(token.into())
            }
        }
    };

    tokens.push(token);
}
