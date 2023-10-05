use std::fmt;

/// Uniquely identifies a syntax fragment
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FragmentId {
    pub(crate) hash: blake3::Hash,
}

impl FragmentId {
    pub fn display_short(&self) -> String {
        self.to_string().split_at(4).0.to_string()
    }
}

impl fmt::Display for FragmentId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}
