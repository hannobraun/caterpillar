use crate::language::repr::{
    eval::{
        fragments::{
            Fragment, FragmentAddress, FragmentId, FragmentPayload, Fragments,
        },
        value::Value,
    },
    syntax::{SyntaxElement, SyntaxTree},
};

pub fn analyze(
    syntax_tree: SyntaxTree,
    fragments: &mut Fragments,
) -> AnalyzerOutput {
    let parent = None;
    let start = analyze_syntax_tree(syntax_tree, parent, fragments);
    AnalyzerOutput { start: Some(start) }
}

fn analyze_syntax_tree(
    syntax_tree: SyntaxTree,
    parent: Option<FragmentId>,
    fragments: &mut Fragments,
) -> FragmentId {
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
    syntax_element: SyntaxElement,
    parent: Option<FragmentId>,
    next: FragmentId,
    fragments: &mut Fragments,
) -> FragmentId {
    let payload = match syntax_element {
        SyntaxElement::Block(syntax_tree) => {
            // By convention, we're using the ID of the *next* fragment as the
            // parent ID for fragments within the block. Why not use the ID of
            // the block fragment itself? Well, that hasn't been computed yet,
            // and it's going to depend on the IDs of the fragments *within* the
            // block, leading to a circular dependency. Won't work!
            let parent = Some(next);

            let start = analyze_syntax_tree(syntax_tree, parent, fragments);
            FragmentPayload::Value(Value::Block { start })
        }
        SyntaxElement::Number(number) => {
            FragmentPayload::Value(Value::Number(number))
        }
        SyntaxElement::Symbol(symbol) => {
            FragmentPayload::Value(Value::Symbol(symbol))
        }
        SyntaxElement::Word(word) => FragmentPayload::Word(word),
    };

    let next = Some(next);
    fragments.insert(Fragment::new(FragmentAddress { parent, next }, payload))
}

pub struct AnalyzerOutput {
    pub start: Option<FragmentId>,
}
