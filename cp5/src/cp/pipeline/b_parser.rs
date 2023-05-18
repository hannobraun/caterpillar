use super::a_tokenizer::Token;

pub fn parse(token: Token) -> Option<String> {
    let Token::Ident(ident) = token;
    Some(ident)
}
