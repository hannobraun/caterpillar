use crate::repr::syntax::{SyntaxElement, SyntaxTree};

pub fn simplify(
    syntax_tree: SyntaxTree<SyntaxElement>,
) -> SyntaxTree<SyntaxElement> {
    // This is a no-op right now. In the future, it will lower some syntax into
    // simplified forms.
    syntax_tree
}
