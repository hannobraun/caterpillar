use crate::repr::{
    eval::{
        fragments::{
            Fragment, FragmentAddress, FragmentId, FragmentPayload, Fragments,
        },
        value::ValuePayload,
    },
    syntax::{SimpleSyntaxElement, SyntaxTree},
};

pub fn analyze(
    syntax_tree: SyntaxTree<SimpleSyntaxElement>,
    parent: Option<FragmentId>,
    fragments: &mut Fragments,
) -> FragmentId {
    analyze_syntax_tree(syntax_tree, parent, fragments)
}

fn analyze_syntax_tree(
    syntax_tree: SyntaxTree<SimpleSyntaxElement>,
    parent: Option<FragmentId>,
    fragments: &mut Fragments,
) -> FragmentId {
    // Add terminator as last fragment within the block. See documentation of
    // `FragmentPayload::Terminator` for an explanation.
    let mut next_fragment = fragments.insert(Fragment::new(
        FragmentAddress { parent, next: None },
        FragmentPayload::Terminator,
    ));

    // We're going through the syntax tree right-to-left here, since the ID of
    // the *next* fragment is part of the address of every fragment (and thus
    // its own ID).
    for syntax_element in syntax_tree.elements.into_iter().rev() {
        next_fragment = analyze_syntax_element(
            syntax_element,
            parent,
            next_fragment,
            fragments,
        );
    }

    next_fragment
}

fn analyze_syntax_element(
    syntax_element: SimpleSyntaxElement,
    parent: Option<FragmentId>,
    next: FragmentId,
    fragments: &mut Fragments,
) -> FragmentId {
    let payload = match syntax_element {
        SimpleSyntaxElement::BlockExpression(syntax_tree) => {
            // By convention, we're using the ID of the *next* fragment as the
            // parent ID for fragments within the block. Why not use the ID of
            // the block fragment itself? Well, that hasn't been computed yet,
            // and it's going to depend on the IDs of the fragments *within* the
            // block, leading to a circular dependency. Won't work!
            let parent = Some(next);

            let start = analyze_syntax_tree(syntax_tree, parent, fragments);
            FragmentPayload::Value(ValuePayload::Block { start })
        }
        SimpleSyntaxElement::Literal(value) => FragmentPayload::Value(value),
        SimpleSyntaxElement::Word(word) => FragmentPayload::Word(word),
    };

    let next = Some(next);
    fragments.insert(Fragment::new(FragmentAddress { parent, next }, payload))
}
