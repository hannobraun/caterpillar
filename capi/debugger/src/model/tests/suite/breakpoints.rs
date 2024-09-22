use std::array;

use crate::model::{
    active_functions::ActiveFunctionsMessage,
    tests::infra::{
        debugger, ActiveFunctionsEntriesExt, ActiveFunctionsExt,
        DebugFunctionExt, FunctionsExt,
    },
    ActiveFunctions, UserAction,
};

#[test]
fn display_breakpoint_that_was_set() -> anyhow::Result<()> {
    // Breakpoints that are set in the debugger state should be displayed.

    let mut debugger = debugger();
    debugger
        .provide_source_code(
            r"
                main: { \ size_x size_y ->
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
        .map(|(id, _)| id)
        .next()
        .unwrap();

    assert!(
        !debugger
            .expect_fragment(&nop.this)
            .data
            .has_durable_breakpoint
    );

    debugger
        .on_user_action(UserAction::BreakpointSet { fragment: nop.this })?;
    assert!(
        debugger
            .expect_fragment(&nop.this)
            .data
            .has_durable_breakpoint
    );

    Ok(())
}

#[test]
fn set_breakpoint_and_stop_there() -> anyhow::Result<()> {
    // If a breakpoint has been set, the program should run up until there, then
    // stop.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: { \ size_x size_y ->
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
        .map(|(id, _)| id)
        .next()
        .unwrap();
    debugger
        .on_user_action(UserAction::BreakpointSet { fragment: nop.this })?;

    debugger.run_program();

    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .with_name("main")
            .active_fragment()
            .hash(),
        nop.this,
    );

    Ok(())
}

#[test]
fn step_over_brk() -> anyhow::Result<()> {
    // When stopped at a `brk` intrinsic, we expect to be able to step over it.

    let mut debugger = debugger();
    debugger
        .provide_source_code(
            r"
                main: { \ size_x size_y ->
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
            .iter(fragments)
            .map(|(_, fragment)| fragment);

        let brk = body.next().unwrap().hash();
        let nop = body.next().unwrap().hash();

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
            .hash(),
        brk,
    );

    debugger.on_user_action(UserAction::StepIn)?;
    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .hash(),
        nop,
    );

    Ok(())
}

#[test]
fn step_over_breakpoints() -> anyhow::Result<()> {
    // Stepping should step over breakpoints, regardless of whether those are
    // durable or ephemeral.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: { \ size_x size_y ->
                nop # a
                nop # b
                nop # c
            }
        ",
    );

    let [a, b, c] = {
        let fragments = debugger.expect_code();
        let mut body = fragments
            .find_function_by_name("main")
            .unwrap()
            .expect_one_branch()
            .iter(fragments);

        array::from_fn(|_| {
            body.find_map(|(id, fragment)| {
                if fragment.as_comment().is_none() {
                    Some(id)
                } else {
                    None
                }
            })
            .unwrap()
        })
    };

    // Set a durable breakpoint at `a`. The program should stop there.
    debugger.on_user_action(UserAction::BreakpointSet { fragment: a.this })?;
    debugger.run_program();
    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .hash(),
        a.this,
    );

    // Step to `b`, over the durable breakpoint. This sets an ephemeral
    // breakpoint there.
    debugger.on_user_action(UserAction::StepIn)?;
    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .hash(),
        b.this,
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
            .find(|fragment| fragment.hash() == a.this)
            .unwrap()
            .data
            .has_durable_breakpoint
    );

    // Step to `c`, over the ephemeral breakpoint.
    debugger.on_user_action(UserAction::StepIn)?;
    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .hash(),
        c.this,
    );

    Ok(())
}

#[test]
fn step_into_function() {
    // When stopping at a function call and then stepping, we expect to land at
    // the first fragment in the function.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: { \ size_x size_y ->
                1 2 f
            }

            # Add some arguments. In case the compiler decides to generate code
            # to handle those, this makes sure we step over that generated code.
            f: {
                \ 1 a ->
                    nop # a

                \ 2 b ->
                    nop # b
            }
        ",
    );

    let (f, a) = {
        let fragments = debugger.expect_code();

        let f = fragments
            .find_function_by_name("main")
            .unwrap()
            .expect_one_branch()
            .iter(fragments)
            .map(|(id, _)| id)
            .nth(2)
            .unwrap()
            .this;
        let a = fragments
            .find_function_by_name("f")
            .unwrap()
            .branches
            .first()
            .unwrap()
            .start;

        (f, a)
    };

    debugger
        .on_user_action(UserAction::BreakpointSet { fragment: f })
        .unwrap();

    debugger.run_program();
    debugger.on_user_action(UserAction::StepIn).unwrap();

    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("f")
            .active_fragment()
            .hash(),
        a.this,
    );
}

#[test]
fn step_out_of_function_if_at_last_fragment() {
    // When stopping at the last fragment in a function and then stepping, we
    // expect to land at the fragment after the function call.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: { \ size_x size_y ->
                f
                nop
            }

            f: { \ ->
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
            .map(|(_, fragment)| fragment)
            .nth(1)
            .unwrap()
            .hash();
        let nop_in_f = fragments
            .find_function_by_name("f")
            .unwrap()
            .expect_one_branch()
            .iter(fragments)
            .map(|(_, fragment)| fragment)
            .next()
            .unwrap()
            .hash();

        (nop_in_main, nop_in_f)
    };

    debugger
        .on_user_action(UserAction::BreakpointSet { fragment: nop_in_f })
        .unwrap();

    debugger.run_program();
    debugger.on_user_action(UserAction::StepIn).unwrap();

    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .hash(),
        nop_in_main,
    );
}

#[test]
fn step_out_of_main_function() {
    // When stopping out of the main function, the process should be finished
    // afterwards.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: { \ size_x size_y ->
                nop
            }
        ",
    );

    let nop = {
        let fragments = debugger.expect_code();

        fragments
            .find_function_by_name("main")
            .unwrap()
            .expect_one_branch()
            .iter(fragments)
            .map(|(_, fragment)| fragment)
            .next()
            .unwrap()
            .hash()
    };

    debugger
        .on_user_action(UserAction::BreakpointSet { fragment: nop })
        .unwrap();
    debugger.run_program();

    debugger.on_user_action(UserAction::StepIn).unwrap();

    assert!(matches!(
        dbg!(debugger.transient_state().active_functions),
        ActiveFunctions::Message {
            message: ActiveFunctionsMessage::ProcessFinished
        }
    ));
}

#[test]
fn step_over_function_call() {
    // When using "Step Over" while stopped at a function call, we expect to
    // step over it.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: { \ size_x size_y ->
                f
                nop
            }

            f: { \ ->
                nop
            }
        ",
    );

    let (f, nop) = {
        let fragments = debugger.expect_code();
        let mut body = fragments
            .find_function_by_name("main")
            .unwrap()
            .expect_one_branch()
            .iter(fragments)
            .map(|(_, fragment)| fragment);

        let f = body
            .find(|fragment| fragment.as_comment().is_none())
            .unwrap()
            .hash();
        let nop = body
            .find(|fragment| fragment.as_comment().is_none())
            .unwrap()
            .hash();

        (f, nop)
    };

    debugger
        .on_user_action(UserAction::BreakpointSet { fragment: f })
        .unwrap();
    debugger.run_program();

    debugger.on_user_action(UserAction::StepOver).unwrap();

    assert_eq!(
        dbg!(debugger.transient_state().active_functions)
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .hash(),
        nop,
    );
}

#[test]
fn step_out_of_function() {
    // When using "Step Out" within a function, we expect to step into the
    // calling function.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: { \ size_x size_y ->
                f
                nop # b
            }

            f: { \ ->
                nop # a
                nop
            }
        ",
    );

    let (a, b) = {
        let fragments = debugger.expect_code();

        let a = fragments
            .find_function_by_name("f")
            .unwrap()
            .expect_one_branch()
            .iter(fragments)
            .map(|(_, fragment)| fragment)
            .next()
            .unwrap()
            .hash();
        let b = fragments
            .find_function_by_name("main")
            .unwrap()
            .expect_one_branch()
            .iter(fragments)
            .map(|(_, fragment)| fragment)
            .nth(1)
            .unwrap()
            .hash();

        (a, b)
    };

    debugger
        .on_user_action(UserAction::BreakpointSet { fragment: a })
        .unwrap();
    debugger.run_program();

    debugger.on_user_action(UserAction::StepOut).unwrap();

    assert_eq!(
        dbg!(debugger.transient_state().active_functions)
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_fragment()
            .hash(),
        b,
    );
}
