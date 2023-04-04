#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Keyword {}

impl Keyword {
    pub fn parse(_: &str) -> Option<Self> {
        None
    }
}
