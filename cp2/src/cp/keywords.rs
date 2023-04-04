#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Keyword {
    Fn,
}

impl Keyword {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "fn" => Some(Self::Fn),
            _ => None,
        }
    }
}
