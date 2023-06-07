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
    String(String),
}
