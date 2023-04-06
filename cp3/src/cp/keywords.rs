#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Keyword {
    Fn,
    Test,
}

impl Keyword {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "fn" => Some(Self::Fn),
            "test" => Some(Self::Test),
            _ => None,
        }
    }
}
