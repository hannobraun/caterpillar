use crate::{
    pipeline::{module::Module, scripts::Scripts},
    platform::Platform,
    repr::eval::fragments::{FragmentId, Fragments},
    runtime::evaluator::{Evaluator, EvaluatorError},
    PlatformFunction,
};

pub fn evaluate(
    _start: FragmentId,
    _fragments: &Fragments,
    _scripts: &Scripts,
) -> Result<Module, EvaluatorError> {
    // This is only a placeholder, and the pipeline essentially stops after the
    // `analyze` step. What should happen here, is the evaluation of the
    // top-level context, to determine which functions and modules were defined.
    //
    // Here's what we could do in that case:
    //
    // - Functions are defined during evaluation. If we were to always evaluate
    //   after analyzing, we would have a full picture of which functions are
    //   defined in the current code.
    //   This is relevant because there's at least one bug in the current
    //   implementation, and by always having a full picture after each update,
    //   we could simplify that implementation and fix the bug.
    //   The known bug, for reference:
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
    // But what we don't want to do, is run the complete program at
    // compile-time. What we need to make this work is a `main` or `start`
    // function (or whatever is appropriate for the platform) for doing the
    // run-time stuff.
    //
    // Once that is in place, and we can evaluate the top-level context here, we
    // can return a data structure here that represents the module defined in
    // the script. That data structure would contain the functions, tests,
    // bindings, etc.

    dbg!(_scripts);

    let module = Module::default();
    let evaluator = Evaluator::<CompileTimePlatform>::new(module);

    let module = evaluator.global_namespace.into_module();
    Ok(module)
}

struct CompileTimePlatform;

impl Platform for CompileTimePlatform {
    type Context = Context;

    fn functions(
    ) -> impl IntoIterator<Item = (PlatformFunction<Self::Context>, &'static str)>
    {
        []
    }
}

struct Context;
