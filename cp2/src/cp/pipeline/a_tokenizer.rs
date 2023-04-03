use crate::cp::tokens::{Token, Tokens};

pub fn tokenize(code: &str) -> Tokens {
    let tokens = code
        .split_whitespace()
        .map(|token| match token {
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
        })
        .collect();
    Tokens(tokens)
}
