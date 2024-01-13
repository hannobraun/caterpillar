use crate::{
    pipeline::stages::b_parser::parse,
    repr::eval::fragments::{FragmentId, Fragments},
};

use super::stages::{
    a_tokenizer::tokenize,
    b_parser::ParserError,
    c_simplifier::simplify,
    d_analyzer::{analyze, AnalyzerOutput},
};

pub fn run(
    code: &str,
    parent: Option<FragmentId>,
    fragments: &mut Fragments,
) -> Result<PipelineOutput, PipelineError> {
    let tokens = tokenize(code);
    let syntax_tree = parse(tokens)?;
    let syntax_tree = simplify(syntax_tree);
    let AnalyzerOutput { start } = analyze(syntax_tree, parent, fragments);

    // The pipeline stops after the `analyze` step. What comes after, is
    // evaluation in some form. Either from the start, or continued evaluation
    // based on the current runtime state.
    //
    // Since we don't have access to that runtime state here, by design, we
    // can't do any evaluation here. We don't have the information to know how.
    //
    // I'm not sure this is the right way to do it, however, and I don't think
    // it needs to be this way. First, let's go into why this might not be the
    // right way to do things:
    //
    // - Functions are defined during evaluation. If we were to always evaluate
    //   after analyzing, we would have a full picture of which functions are
    //   defined in the current code.
    //   This is relevant because there's at least one bugs in the current
    //   implementation, and by always having a full picture after each update,
    //   we could simplify that implementation and fix the bug.
    //   The known bug, for references:
    //   https://github.com/hannobraun/caterpillar/issues/15
    // - Same goes for modules. There are probably more holes in the current
    //   concept, that will be exposed as Caterpillar's functionality gets
    //   closer to that of a useful language.
    // - Longer-term, I'd like to have powerful compile-time metaprogramming,
    //   and I'd like to use that to implement a static type system in the
    //   language itself. Who knows how realistic that is, but if something like
    //   that were to happen, it sounds reasonable to define that everything in
    //   the top-level context happens at compile-time.
    //
    // So, if we were to immediately evaluate the top-level, this would have
    // immediate and possible future benefits. But it can't work within the
    // current model.
    //
    // There's a relatively straight-forward solution, however (at least I
    // think): Have a `main` or `start` function (or whatever is appropriate for
    // the platform) for doing the run-time stuff. Leave the top-level context
    // for compile-time stuff that we can safely evaluate again and again,
    // without directly disturbing the running program.
    //
    // If that were to happen, we'd add an evaluation stage here, and in
    // addition to the new fragments, would also return a structure that
    // describes all functions, modules, and such.

    Ok(PipelineOutput { start })
}

pub struct PipelineOutput {
    pub start: FragmentId,
}

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Failed to parse")]
    Parser(#[from] ParserError),
}
