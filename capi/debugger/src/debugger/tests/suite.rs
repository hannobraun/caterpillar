use capi_process::{CoreEffect, Effect};

use crate::debugger::{
    active_functions::ActiveFunctionsMessage,
    tests::infra::{
        init, ActiveFunctionsExt, ExpressionExt, FragmentExpressionExt,
    },
    ActiveFunctions,
};

#[test]
fn no_server() {
    // If `RemoteProcess` has received no updates at all, the active functions
    // view should display that no server is available.

    let debugger = init().to_debugger();

    assert_eq!(
        debugger.active_functions,
        ActiveFunctions::Message {
            message: ActiveFunctionsMessage::NoServer
        }
    );
    assert!(debugger.operands.is_none());
    assert!(debugger.memory.is_none());
}

#[test]
fn no_process() {
    // If `RemoteProcess` has received a code update but no runtime updates, the
    // active functions view should display that no process is available.

    let debugger = init().provide_source_code(|_| {}).to_debugger();

    assert_eq!(
        debugger.active_functions,
        ActiveFunctions::Message {
            message: ActiveFunctionsMessage::NoProcess
        }
    );
    assert!(debugger.operands.is_none());
    assert!(debugger.memory.is_none());
}

#[test]
fn basic_call_stack() {
    // All functions in the call stack should show up in the active functions
    // view, if the process is stopped. This test constructs a scenario that
    // requires no special handling to detect and fix tail call optimization, to
    // provide a baseline.
    //
    // This test expects all defined functions to be active functions. The order
    // of functions is inner to outer, as it's most useful to the developer to
    // display the instruction where we're currently paused up top.

    let debugger = init()
        .provide_source_code(|script| {
            script
                .function("main", [], |s| {
                    s.r("f")
                        // Not triggered. Just here to prevent tail call
                        // optimization from removing this function from the
                        // call stack.
                        .r("brk");
                })
                .function("f", [], |s| {
                    s.r("g")
                        // Not triggered. Just here to prevent tail call
                        // optimization from removing this function from the
                        // call stack.
                        .r("brk");
                })
                .function("g", [], |s| {
                    s.r("brk");
                });
        })
        .run_process()
        .to_debugger();

    let names = debugger
        .active_functions
        .expect_functions()
        .into_iter()
        .map(|active_function| active_function.name)
        .collect::<Vec<_>>();
    assert_eq!(names, vec!["g", "f", "main"]);
}

#[test]
fn stopped_at_code_within_block() {
    // If execution is stopped within a block, the function that contains that
    // block should appear as an active function, and the current instruction
    // should be visible.

    let debugger = init()
        .provide_source_code(|script| {
            script.function("main", [], |s| {
                s.block(|s| {
                    s.r("brk");
                })
                .r("eval");
            });
        })
        .run_process()
        .to_debugger();

    let other = debugger
        .active_functions
        .expect_functions()
        .remove(0)
        .body
        .remove(0)
        .expect_block()
        .remove(0)
        .expect_other();
    assert_eq!(other.effect, Some(Effect::Core(CoreEffect::Breakpoint)));

    let builtin = other.expression.expect_builtin_function();
    assert_eq!(builtin, "brk");
}
