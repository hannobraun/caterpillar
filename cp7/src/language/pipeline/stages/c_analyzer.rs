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
            // By convention, we're using the ID of the *next* fragment as the
            // parent ID for fragments within the block. Why not use the ID of
            // the block fragment itself? Well, that hasn't been computed yet,
            // and it's going to depend on the IDs of the fragments *within* the
            // block, leading to a circular dependency. Won't work!
            //
            // In principle, we could instead use the address of the *previous*
            // fragment. That would serve just as well to uniquely address the
            // fragments within the block. However, since we use the ID of the
            // next fragment for addressing all fragments, that one is easily
            // available to us right here and now, while the ID of the previous
            // fragment is not known yet.
            //
            // You might ask, how will this work for blocks that are in the last
            // position within their context (which is either the top-level
            // context or a parent block). Won't the fragments within such a
            // block not all have a `None` parent? Will this not mean, that such
            // blocks are indistinguishable, if their contents are identical?
            //
            // Yes, indeed. The solution I have in mind is to introduce a
            // terminator fragment at the end of each context. Those will have
            // an ID that depends on their parents. Since there can only be one
            // block at the end of each context, that means every block will be
            // uniquely addressed again, regardless of whether it's identical to
            // any other blocks.
            let parent = next;

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

    fragments.add(Fragment::new(FragmentAddress { parent, next }, payload))
}

pub struct AnalyzerOutput {
    pub start: Option<FragmentId>,
}
