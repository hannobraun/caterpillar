use super::eval::value::ValuePayload;

#[derive(Clone, Debug)]
pub struct SyntaxTree<T> {
    pub elements: Vec<T>,
}

impl<T> SyntaxTree<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Default for SyntaxTree<T> {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SyntaxElement {
    ArrayExpression(SyntaxTree<Self>),
    BlockExpression(SyntaxTree<Self>),

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

pub enum SimpleSyntaxElement {
    BlockExpression(SyntaxTree<Self>),
    Literal(ValuePayload),
    Word(String),
}
