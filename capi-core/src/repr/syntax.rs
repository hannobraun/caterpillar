use super::eval::value::ValuePayload;

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    pub elements: Vec<SyntaxElement>,
}

impl SyntaxTree {
    pub fn new() -> Self {
        Self::default()
    }
}

#[allow(clippy::derivable_impls)]
impl Default for SyntaxTree {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    Array(SyntaxTree),
    Block(SyntaxTree),

    /// A literal value
    ///
    /// This variant can represent `SyntaxElement`s that are not actually valid,
    /// as [`ValueKind`] can be a block, but a block is actually handled by a
    /// dedicated variant.
    ///
    /// Such an invalid `SyntaxElement` is never produced by the parser, and
    /// doing it like this makes the code handling `SyntaxElement`s simpler.
    /// That is probably worth the small inconsistency.
    Literal(ValuePayload),

    Word(String),
}
