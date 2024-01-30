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
    start: FragmentId,
    fragments: &mut Fragments,
    scripts: &Scripts,
) -> Result<Module, EvaluatorError> {
    let module = Module::default();
    let mut evaluator = Evaluator::<CompileTimePlatform>::new(module);

    let mut context = Context { scripts };

    evaluator
        .call_stack
        .push(StackFrame::Fragment { fragment_id: start });
    while !evaluator.step(fragments, &mut context)?.finished() {}

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
