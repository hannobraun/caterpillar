use crate::model::{
    tests::infra::{
        debugger, ActiveFunctionsEntriesExt, ActiveFunctionsExt,
        DebugFunctionExt, FunctionsExt,
    },
    UserAction,
};

#[test]
fn display_breakpoint_that_was_set() -> anyhow::Result<()> {
    // Breakpoints that are set in the debugger state should be displayed.

    let mut debugger = debugger();
    debugger
        .provide_source_code(
            r"
                main: { |size_x size_y|
                    nop # this is where the breakpoint will be set
                    brk # prevent process from ending before we set breakpoint
                }
            ",
        )
        .run_program();

    let fragments = debugger.expect_code();
    let nop = fragments
        .find_function_by_name("main")
        .unwrap()
        .expect_one_branch()
        .iter(fragments)
        .next()
        .unwrap()
        .id();

    assert!(!debugger.expect_fragment(&nop).data.has_durable_breakpoint);

    debugger.on_user_action(UserAction::BreakpointSet { fragment: nop })?;
    assert!(debugger.expect_fragment(&nop).data.has_durable_breakpoint);

    Ok(())
}

#[test]
fn set_breakpoint_and_stop_there() -> anyhow::Result<()> {
    // If a breakpoint has been set, the program should run up until there, then
    // stop.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: { |size_x size_y|
                nop
            }
        ",
    );

    let fragments = debugger.expect_code();
    let nop = fragments
        .find_function_by_name("main")
        .unwrap()
        .expect_one_branch()
        .iter(fragments)
        .next()
        .unwrap()
        .id();
    debugger.on_user_action(UserAction::BreakpointSet { fragment: nop })?;

    debugger.run_program();

    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .with_name("main")
            .active_fragment()
            .data
            .id,
        nop,
    );

    Ok(())
}

#[test]
fn step_over_brk() {
    // When stopped at a `brk` intrinsic, we expect to be able to step over it.

    let mut debugger = debugger();
    debugger
        .provide_source_code(
            r"
            main: { |size_x size_y|
                brk
                nop
            }
        ",
        )
        .run_program();

    let (brk, nop) = {
        let fragments = debugger.expect_code();
        let mut body = fragments
            .find_function_by_name("main")
            .unwrap()
            .expect_one_branch()
            .iter(fragments);

        let brk = body.next().unwrap().id();
        let nop = body.next().unwrap().id();

        (brk, nop)
    };

    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .data
            .id,
        brk,
    );

    debugger.on_user_action(UserAction::StepInto).unwrap();
    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .data
            .id,
        nop,
    );
}

#[test]
fn step_over_breakpoints() -> anyhow::Result<()> {
    // Stepping should step over breakpoints, regardless of whether those are
    // durable or ephemeral.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: { |size_x size_y|
                nop # a
                nop # b
                nop # c
            }
        ",
    );

    let (a, b, c) = {
        let fragments = debugger.expect_code();
        let mut body = fragments
            .find_function_by_name("main")
            .unwrap()
            .expect_one_branch()
            .iter(fragments);

        let a = body.find(|fragment| !fragment.is_comment()).unwrap().id();
        let b = body.find(|fragment| !fragment.is_comment()).unwrap().id();
        let c = body.find(|fragment| !fragment.is_comment()).unwrap().id();

        (a, b, c)
    };

    // Set a durable breakpoint at `a`. The program should stop there.
    debugger.on_user_action(UserAction::BreakpointSet { fragment: a })?;
    debugger.run_program();
    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .data
            .id,
        a,
    );

    // Step to `b`, over the durable breakpoint. This sets an ephemeral
    // breakpoint there.
    debugger.on_user_action(UserAction::StepInto)?;
    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .data
            .id,
        b,
    );
    assert!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_branch()?
            .body
            .iter()
            .find(|fragment| fragment.data.id == a)
            .unwrap()
            .data
            .has_durable_breakpoint
    );

    // Step to `c`, over the ephemeral breakpoint.
    debugger.on_user_action(UserAction::StepInto)?;
    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .data
            .id,
        c,
    );

    Ok(())
}

#[test]
#[should_panic] // https://github.com/hannobraun/caterpillar/issues/52
fn step_into_function() {
    // When stopping at a function call and then stepping, we expect to land at
    // the first fragment in the function.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: { |size_x size_y|
                1 2 3 4 f
            }

            # Add some arguments. In case the compiler decides to generate code
            # to handle those, this makes sure we step over that generated code.
            f: { |1 a 3 b|
                nop
            }
        ",
    );

    let f = {
        let fragments = debugger.expect_code();
        fragments
            .find_function_by_name("main")
            .unwrap()
            .expect_one_branch()
            .iter(fragments)
            .nth(4)
            .unwrap()
            .id()
    };

    debugger
        .on_user_action(UserAction::BreakpointSet { fragment: f })
        .unwrap();

    debugger.run_program();
    debugger.on_user_action(UserAction::StepInto).unwrap();

    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("f")
            .active_fragment()
            .data
            .id,
        f,
    );
}

#[test]
#[should_panic] // https://github.com/hannobraun/caterpillar/issues/24
                // https://github.com/hannobraun/caterpillar/issues/52
fn step_out_of_function() {
    // When stopping at the last fragment in a function and then stepping, we
    // expect to land at the fragment after the function call.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: { |size_x size_y|
                f
                nop
            }

            f: { ||
                nop
                # There's a return instruction at the end of the function, which
                # we expect to step over.
            }
        ",
    );

    let (nop_in_main, nop_in_f) = {
        let fragments = debugger.expect_code();

        let nop_in_main = fragments
            .find_function_by_name("main")
            .unwrap()
            .expect_one_branch()
            .iter(fragments)
            .nth(1)
            .unwrap()
            .id();
        let nop_in_f = fragments
            .find_function_by_name("f")
            .unwrap()
            .expect_one_branch()
            .iter(fragments)
            .next()
            .unwrap()
            .id();

        (nop_in_main, nop_in_f)
    };

    debugger
        .on_user_action(UserAction::BreakpointSet { fragment: nop_in_f })
        .unwrap();

    debugger.run_program();
    debugger.on_user_action(UserAction::StepInto).unwrap();

    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .data
            .id,
        nop_in_main,
    );
}
