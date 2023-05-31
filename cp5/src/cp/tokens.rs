#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    CurlyBracketOpen,
    CurlyBracketClose,
    Keyword(Keyword),
    Ident(String),
    String(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    Fn,
    Mod,
    Test,
}
