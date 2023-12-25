use crate::repr::syntax::{SimpleSyntaxElement, SyntaxElement, SyntaxTree};

pub fn simplify(
    syntax_tree: SyntaxTree<SyntaxElement>,
) -> SyntaxTree<SimpleSyntaxElement> {
    simplify_syntax_tree(syntax_tree)
}

fn simplify_syntax_tree(
    syntax_tree: SyntaxTree<SyntaxElement>,
) -> SyntaxTree<SimpleSyntaxElement> {
    let mut simple_syntax_tree = SyntaxTree::new();

    for syntax_element in syntax_tree.elements {
        let syntax_element = match syntax_element {
            SyntaxElement::Array(syntax_tree) => {
                let syntax_tree = simplify_array(syntax_tree);
                SimpleSyntaxElement::Array(syntax_tree)
            }
            SyntaxElement::Block(syntax_tree) => {
                let syntax_tree = simplify_block(syntax_tree);
                SimpleSyntaxElement::Block(syntax_tree)
            }
            SyntaxElement::Literal(value) => {
                SimpleSyntaxElement::Literal(value)
            }
            SyntaxElement::Word(word) => SimpleSyntaxElement::Word(word),
        };

        simple_syntax_tree.elements.push(syntax_element);
    }

    simple_syntax_tree
}

fn simplify_array(
    syntax_tree: SyntaxTree<SyntaxElement>,
) -> SyntaxTree<SimpleSyntaxElement> {
    // This is a no-op right now. In the future, it will lower array expressions
    // into a simplified form.
    simplify_syntax_tree(syntax_tree)
}

fn simplify_block(
    syntax_tree: SyntaxTree<SyntaxElement>,
) -> SyntaxTree<SimpleSyntaxElement> {
    simplify_syntax_tree(syntax_tree)
}
