use std::marker::PhantomData;

use crate::{
    pipeline::{self, module::Module, scripts::Scripts, PipelineOutput},
    platform::Platform,
    repr::eval::fragments::{FragmentId, Fragments},
    runtime::{
        call_stack::StackFrame,
        evaluator::{Evaluator, EvaluatorError},
    },
    value, DataStackResult, PlatformFunction, PlatformFunctionState,
    RuntimeContext,
};

pub fn evaluate(
    _start: FragmentId,
    _fragments: &mut Fragments,
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
    let mut evaluator = Evaluator::<CompileTimePlatform>::new(module);

    let mut context = Context { scripts: _scripts };

    evaluator.call_stack.push(StackFrame::Fragment {
        fragment_id: _start,
    });
    while !evaluator.step(_fragments, &mut context)?.finished() {}

    let module = evaluator.global_namespace.into_module();
    Ok(module)
}

struct CompileTimePlatform<'r> {
    // We need a lifetime here, so we have one available for `Context` in the
    // `Platform` trait implementation. Not sure if there's a better way, but
    // this seems to work.
    _r: PhantomData<&'r ()>,
}

impl<'r> Platform for CompileTimePlatform<'r> {
    type Context = Context<'r>;

    fn functions(
    ) -> impl IntoIterator<Item = (PlatformFunction<Self::Context>, &'static str)>
    {
        [(mod_ as PlatformFunction<Self::Context>, "mod")]
    }
}

struct Context<'r> {
    scripts: &'r Scripts,
}

fn mod_(
    runtime_context: RuntimeContext,
    platform_context: &mut Context,
) -> DataStackResult<PlatformFunctionState> {
    let (path_as_values, _) =
        runtime_context.data_stack.pop_specific::<value::Array>()?;

    let mut path = Vec::new();
    for value in path_as_values.0 {
        let symbol = value.expect::<value::Symbol>()?;
        path.push(symbol);
    }

    // The error handling here is not great, since the return value of platform
    // functions doesn't give us a lot of flexibility.
    //
    // We'll need a platform-specific return value before long, probably, but
    // this will do for now.
    let code = platform_context.scripts.inner.get(&path).unwrap();

    // The error handling here is not great, since the return value of platform
    // functions doesn't give us a lot of flexibility.
    //
    // We'll need a platform-specific return value before long, probably, but
    // this will do for now.
    let parent = Some(runtime_context.word);
    let PipelineOutput { mut module, .. } = pipeline::run(
        code,
        parent,
        runtime_context.fragments,
        platform_context.scripts,
    )
    .unwrap();

    // Eventually, we'd want to add `module` as a child to the existing module.
    // For now, everything lives in a single global namespace, so we just merge
    // the two modules together.
    runtime_context
        .namespace
        .bindings
        .append(&mut module.bindings);
    runtime_context
        .namespace
        .functions
        .0
        .append(&mut module.functions.0);
    runtime_context
        .namespace
        .tests
        .0
        .append(&mut module.tests.0);

    Ok(PlatformFunctionState::Done)
}
