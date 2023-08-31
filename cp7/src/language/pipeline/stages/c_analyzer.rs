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
    AnalyzerOutput { start }
}

fn analyze_syntax_tree(
    syntax_tree: SyntaxTree,
    parent: Option<FragmentId>,
    fragments: &mut Fragments,
) -> Option<FragmentId> {
    let mut next_fragment = None;

    // We're going through the syntax tree right-to-left here, since the ID of
    // the *next* fragment is part of the address of every fragment (and thus
    // its own ID).
    for syntax_element in syntax_tree.elements.into_iter().rev() {
        next_fragment = Some(analyze_syntax_element(
            syntax_element,
            parent,
            next_fragment,
            fragments,
        ));
    }

    next_fragment
}

fn analyze_syntax_element(
    syntax_element: SyntaxElement,
    parent: Option<FragmentId>,
    next: Option<FragmentId>,
    fragments: &mut Fragments,
) -> FragmentId {
    let payload = match syntax_element {
        SyntaxElement::Block(syntax_tree) => {
            let start = analyze_syntax_tree(syntax_tree, next, fragments);
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

    fragments.add(Fragment::new(FragmentAddress { parent, next }, payload))
}

pub struct AnalyzerOutput {
    pub start: Option<FragmentId>,
}
