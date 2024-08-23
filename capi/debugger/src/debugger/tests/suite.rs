use capi_compiler::{
    fragments::{FragmentKind, Payload},
    intrinsics::Intrinsic,
};
use capi_process::Effect;

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
    assert!(debugger.operands.is_empty());
    assert!(debugger.memory.is_none());
}

#[test]
fn no_process() {
    // If `RemoteProcess` has received a code update but no runtime updates, the
    // active functions view should display that no process is available.

    let debugger = init().provide_source_code("").to_debugger();

    assert_eq!(
        debugger.active_functions,
        ActiveFunctions::Message {
            message: ActiveFunctionsMessage::NoProcess
        }
    );
    assert!(debugger.operands.is_empty());
    assert!(debugger.memory.is_none());
}

#[test]
fn basic_call_stack() {
    // All functions in the call stack should show up in the active functions
    // view, if the process is stopped. This test constructs a scenario that
    // requires no special handling to detect and fix the effects of tail call
    // elimination, to provide a baseline.
    //
    // This test expects all defined functions to be active functions. The order
    // of functions is inner to outer, as it's most useful to the developer to
    // display the instruction where we're currently paused up top.

    let debugger = init()
        .provide_source_code(
            r"
            main: { |size_x size_y|
                f
                nop # make sure the previous call is not a tail call
            }
            f: { ||
                g
                nop # make sure the previous call is not a tail call
            }
            g: { ||
                brk
            }",
        )
        .run_process()
        .to_debugger();

    let names = debugger
        .active_functions
        .expect_functions()
        .into_iter()
        .map(|active_function| active_function.name.unwrap())
        .collect::<Vec<_>>();
    assert_eq!(names, vec!["g", "f", "main"]);
}

#[test]
fn stopped_at_code_within_block() {
    // If execution is stopped within a block, the function that contains that
    // block should appear as an active function, and the current instruction
    // should be visible.

    let debugger = init()
        .provide_source_code(
            r"
                main: { |size_x size_y|
                    { || brk } eval
                }
            ",
        )
        .run_process()
        .to_debugger();

    let expression = debugger
        .active_functions
        .expect_functions()
        .remove(0)
        .branches
        .remove(0)
        .body
        .remove(0)
        .expect_block()
        .remove(0)
        .expect_other();
    assert_eq!(expression.effect, Some(Effect::Breakpoint));

    let intrinsic = expression.expression.expect_intrinsic();
    assert_eq!(intrinsic, Intrinsic::Brk);
}

#[test]
fn call_stack_reconstruction_missing_main() {
    // Tail call elimination can leave gaps in the call stack. If the `main`
    // function is missing due to that, it should be reconstructed.

    let debugger = init()
        .provide_source_code(
            r"
                main: { |size_x size_y|
                    f
                }

                f: { ||
                    brk
                }
            ",
        )
        .run_process()
        .to_debugger();

    let names = debugger.active_functions.names();
    assert_eq!(names, vec!["f", "main"]);

    let tail_call = {
        let id = debugger
            .active_functions
            .expect_functions()
            .last()
            .unwrap()
            .active_fragment
            .unwrap();
        let code = debugger.code.unwrap();
        code.fragments.inner.inner.get(&id).unwrap().clone()
    };
    let FragmentKind::Payload {
        payload: Payload::CallToFunction { name, .. },
        ..
    } = tail_call.kind
    else {
        panic!()
    };
    assert_eq!(name, "f");
}
