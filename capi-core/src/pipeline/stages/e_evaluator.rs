use std::marker::PhantomData;

use crate::{
    builtins::types::{BuiltinContext, PlatformBuiltin, PlatformBuiltinState},
    pipeline::{
        self, module::Module, scripts::Scripts, FunctionName, PipelineOutput,
    },
    platform::{Platform, PlatformBuiltins},
    repr::eval::{
        fragments::{FragmentId, Fragments},
        value,
    },
    runtime::{
        call_stack::StackFrame,
        data_stack::DataStackResult,
        evaluator::{Evaluator, EvaluatorError},
    },
};

pub fn evaluate(
    start: FragmentId,
    fragments: &mut Fragments,
    scripts: &Scripts,
) -> Result<Module, EvaluatorError> {
    // This function evaluates the top-level context at compile-time. The way
    // that is implemented, means modules are implicit, and there are platform
    // functions like `fn` and `mod` that update this implicit module.
    //
    // I'd prefer it, if modules where actually explicit data structures, `fn`
    // and `mod` would create anonymous functions and modules explicitly, and
    // those anonymous items were named by adding them into named fields of the
    // module.
    //
    // Then a module, as written in the code, would just become a function,
    // evaluated at compile-time, that returns such a module data structure.

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

    fn builtin_fns() -> impl PlatformBuiltins<Self> {
        [(fn_ as PlatformBuiltin<Self>, "fn"), (mod_, "mod")]
    }
}

struct Context<'r> {
    scripts: &'r Scripts,
}

fn fn_(
    runtime_context: BuiltinContext,
    _platform_context: &mut Context,
) -> DataStackResult<PlatformBuiltinState> {
    let (body, _) =
        runtime_context.data_stack.pop_specific::<value::Block>()?;
    let (name, name_fragment) =
        runtime_context.data_stack.pop_specific::<value::Symbol>()?;

    let name = FunctionName {
        value: name.0,
        fragment: name_fragment,
    };

    runtime_context.global_module.define_function(name, body);

    Ok(PlatformBuiltinState::Done)
}

fn mod_(
    runtime_context: BuiltinContext,
    platform_context: &mut Context,
) -> DataStackResult<PlatformBuiltinState> {
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
    runtime_context.global_module.merge(&mut module);

    Ok(PlatformBuiltinState::Done)
}
