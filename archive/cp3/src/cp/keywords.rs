#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Keyword {
    Fn,
    Mod,
    Test,
}

impl Keyword {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "fn" => Some(Self::Fn),
            "mod" => Some(Self::Mod),
            "test" => Some(Self::Test),
            _ => None,
        }
    }
}
