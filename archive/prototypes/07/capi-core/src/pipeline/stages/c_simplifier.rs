use crate::repr::{
    eval::value::ValuePayload,
    syntax::{SimpleSyntaxElement, SyntaxElement, SyntaxTree},
};

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
            SyntaxElement::ArrayExpression(syntax_tree) => {
                let syntax_elements = simplify_array(syntax_tree);
                simple_syntax_tree.elements.extend(syntax_elements);
                continue;
            }
            SyntaxElement::Binding { names } => {
                let syntax_elements = simplify_binding(names);
                simple_syntax_tree.elements.extend(syntax_elements);
                continue;
            }
            SyntaxElement::BlockExpression(syntax_tree) => {
                let syntax_tree = simplify_block(syntax_tree);
                SimpleSyntaxElement::BlockExpression(syntax_tree)
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
) -> [SimpleSyntaxElement; 3] {
    let syntax_tree = simplify_syntax_tree(syntax_tree);
    make_simple_array_expression(syntax_tree)
}

fn simplify_binding(names: Vec<String>) -> [SimpleSyntaxElement; 4] {
    let syntax_tree = SyntaxTree {
        elements: names
            .into_iter()
            .map(|name| {
                SimpleSyntaxElement::Literal(ValuePayload::Symbol(name))
            })
            .collect(),
    };
    let [a, b, c] = make_simple_array_expression(syntax_tree);
    [a, b, c, SimpleSyntaxElement::Word(String::from("bind"))]
}

fn simplify_block(
    syntax_tree: SyntaxTree<SyntaxElement>,
) -> SyntaxTree<SimpleSyntaxElement> {
    simplify_syntax_tree(syntax_tree)
}

fn make_simple_array_expression(
    syntax_tree: SyntaxTree<SimpleSyntaxElement>,
) -> [SimpleSyntaxElement; 3] {
    [
        SimpleSyntaxElement::Word(String::from("[]")),
        SimpleSyntaxElement::BlockExpression(syntax_tree),
        SimpleSyntaxElement::Word(String::from("append")),
    ]
}
