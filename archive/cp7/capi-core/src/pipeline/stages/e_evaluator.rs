use crate::{
    pipeline::{self, module::Module, scripts::Scripts, PipelineOutput},
    platform::{
        BuiltinFn, BuiltinFnResult, BuiltinFnState, BuiltinFns, CoreContext,
        Platform,
    },
    repr::eval::{
        fragments::{FragmentId, Fragments},
        value,
    },
    runtime::{
        call_stack::StackFrame,
        evaluator::{Evaluator, EvaluatorError},
    },
};

// The evaluation here requires that all reachable module have already been
// loaded. This seemed like a good idea at the time, at least as a stop-gap, but
// I think it's the wrong approach.
//
// Not only has it turned out to be surprisingly tricky to implement, it's also
// limited: If a new file is added, this either needs to be detected somehow
// (which could end up in a race condition with the new file being needed), or
// we need some way for the pipeline to ask for it anyway. And then the whole
// preloading bit is pointless.
//
// It probably makes sense to create a `Pipeline` struct that can hold the
// current state of evaluation, while the loader goes off to load whatever was
// missing.

pub fn evaluate(
    start: FragmentId,
    fragments: &mut Fragments,
    scripts: &Scripts,
) -> Result<Module, EvaluatorError<()>> {
    // This function evaluates the top-level context at compile-time. In the
    // current implementation, modules are implicit, and there are builtin
    // functions like `fn` and `mod` that update these implicit modules.
    //
    // It would be nicer, if modules where actually explicit data structures,
    // `fn` and `mod` would create anonymous functions and modules respectively,
    // and those anonymous items were named by adding them into named fields of
    // the module data structure.
    //
    // Then a module, as written in the code, would just become a function,
    // evaluated at compile-time, that returns such a module data structure.

    let module = Module::default();
    let mut evaluator = Evaluator::<CompileTimePlatform>::new(module);

    evaluator
        .call_stack
        .push(StackFrame::Fragment { fragment_id: start });
    while !evaluator
        .step(fragments, &mut PlatformContext { scripts })?
        .finished()
    {}

    let module = evaluator.global_namespace.into_module();
    Ok(module)
}

struct CompileTimePlatform;

impl Platform for CompileTimePlatform {
    type Context<'r> = PlatformContext<'r>;
    type Error = ();

    fn builtin_fns() -> impl BuiltinFns<Self> {
        [
            (fn_ as BuiltinFn<Self>, "fn"),
            (mod_, "mod"),
            (test, "test"),
        ]
    }
}

struct PlatformContext<'r> {
    scripts: &'r Scripts,
}

fn fn_(
    step: usize,
    runtime_context: CoreContext,
    _platform_context: &mut PlatformContext,
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (body, _) =
                runtime_context.data_stack.pop_specific::<value::Block>()?;
            let (name, _) =
                runtime_context.data_stack.pop_specific::<value::Symbol>()?;

            runtime_context.global_module.define_function(name.0, body);

            Ok(BuiltinFnState::Completed)
        }

        _ => unreachable!(),
    }
}

fn mod_(
    step: usize,
    runtime_context: CoreContext,
    platform_context: &mut PlatformContext,
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (path_as_values, _) =
                runtime_context.data_stack.pop_specific::<value::Array>()?;

            let mut path = Vec::new();
            for value in path_as_values.0 {
                let symbol = value.expect::<value::Symbol>()?;
                path.push(symbol);
            }

            // The error handling here is not great, since the return value of
            // platform functions doesn't give us a lot of flexibility.
            //
            // We'll need a platform-specific return value before long,
            // probably, but this will do for now.
            let code = platform_context.scripts.inner.get(&path).unwrap();

            // The error handling here is not great, since the return value of
            // platform functions doesn't give us a lot of flexibility.
            //
            // We'll need a platform-specific return value before long,
            // probably, but this will do for now.
            let parent = Some(runtime_context.word);
            let PipelineOutput { mut module, .. } = pipeline::run(
                code,
                parent,
                runtime_context.fragments,
                platform_context.scripts,
            )
            .unwrap();

            // Eventually, we'd want to add `module` as a child to the existing
            // module. For now, everything lives in a single global namespace,
            // so we just merge the two modules together.
            runtime_context.global_module.merge(&mut module);

            Ok(BuiltinFnState::Completed)
        }
        _ => unreachable!(),
    }
}

fn test(
    step: usize,
    context: CoreContext,
    _platform_context: &mut PlatformContext,
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (body, _) =
                context.data_stack.pop_specific::<value::Block>()?;
            let (name, _) = context.data_stack.pop_specific::<value::Text>()?;

            context.global_module.define_test(name.0, body);

            Ok(BuiltinFnState::Completed)
        }
        _ => unreachable!(),
    }
}
