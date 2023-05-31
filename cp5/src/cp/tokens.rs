#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    CurlyBracketOpen,
    CurlyBracketClose,
    Fn,
    Mod,
    Test,
    Ident(String),
    String(String),
}
