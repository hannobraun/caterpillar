#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    BindingOperator,
    Period,
    CurlyBracketOpen,
    CurlyBracketClose,
    SquareBracketOpen,
    SquareBracketClose,
    Keyword(Keyword),
    Literal(Literal),
    Ident(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    Fn,
    Mod,
    Test,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    Number(u8),
    String(String),
}

pub const STRING_DELIMITER: char = '"';

pub const DELIMITERS: &[(&str, Token)] = &[
    ("=>", Token::BindingOperator),
    (".", Token::Period),
    ("{", Token::CurlyBracketOpen),
    ("}", Token::CurlyBracketClose),
    ("[", Token::SquareBracketOpen),
    ("]", Token::SquareBracketClose),
];

pub const KEYWORDS: &[(&str, Token)] = &[
    ("fn", Token::Keyword(Keyword::Fn)),
    ("mod", Token::Keyword(Keyword::Mod)),
    ("test", Token::Keyword(Keyword::Test)),
];
