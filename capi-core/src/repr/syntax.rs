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
    Binding {
        names: Vec<String>,
    },
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
    // I'm wondering if it's possible to lower blocks expressions. Something
    // like this:
    //
    // ```
    // { a b c }
    // ```
    //
    // Into this:
    //
    // ```
    // [] :a push :b push :c push
    // ```
    //
    // The one problem with this example (except that there's no `push`, which
    // is trivial to fix) is that the result would be an array of symbols, not
    // a block. Meaning the runtime wouldn't know that this is a block.
    //
    // I see two ways to address that:
    //
    // 1. Don't. Have arrays of symbols take the place of blocks everywhere. Not
    //    desirable long term, due to the weak typing, but I don't see why it
    //    wouldn't work for the time being.
    // 2. Define a "block" type which wraps the array. `push` would work on it.
    //    More desirable as a long-term solution, but unless I'm going to
    //    hardcode this (which would seem to defeat the point of lowering block
    //    expressions), it's going to require the runtime to understand types
    //    that the user can define.
    BlockExpression(SyntaxTree<Self>),
    Literal(ValuePayload),
    Word(String),
}
