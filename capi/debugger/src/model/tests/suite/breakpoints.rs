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
                main: fn \ size_x, size_y ->
                    nop # this is where the breakpoint will be set
                    brk # prevent process from ending before we set breakpoint
                end
            ",
        )
        .run_program();

    let named_functions = debugger.expect_code();
    let nop = named_functions
        .find_by_name("main")
        .unwrap()
        .find_single_branch()
        .unwrap()
        .body()
        .next()
        .unwrap()
        .into_location();

    assert!(!debugger.expect_fragment(&nop).data.has_durable_breakpoint);

    debugger.on_user_action(UserAction::BreakpointSet {
        fragment: nop.clone(),
    })?;
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
            main: fn \ size_x, size_y ->
                nop
            end
        ",
    );

    let named_functions = debugger.expect_code();
    let nop = named_functions
        .find_by_name("main")
        .unwrap()
        .find_single_branch()
        .unwrap()
        .body()
        .next()
        .unwrap()
        .into_location();
    debugger.on_user_action(UserAction::BreakpointSet {
        fragment: nop.clone(),
    })?;

    debugger.run_program();

    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .with_name("main")
            .active_expression()
            .data
            .location,
        nop,
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
                main: fn \ size_x, size_y ->
                    brk
                    nop
                end
            ",
        )
        .run_program();

    let [brk, nop] = {
        let named_functions = debugger.expect_code();
        let mut body = named_functions
            .find_by_name("main")
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body()
            .map(|fragment| fragment.into_location());

        array::from_fn(|_| body.next().unwrap())
    };

    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_expression()
            .data
            .location,
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
            .active_expression()
            .data
            .location,
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
            main: fn \ size_x, size_y ->
                nop # a
                nop # b
                nop # c
            end
        ",
    );

    let [a, b, c] = {
        let named_functions = debugger.expect_code();
        let mut body = named_functions
            .find_by_name("main")
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body();

        array::from_fn(|_| {
            body.find_map(|fragment| {
                if fragment.as_comment().is_none() {
                    Some(fragment.into_location())
                } else {
                    None
                }
            })
            .unwrap()
        })
    };

    // Set a durable breakpoint at `a`. The program should stop there.
    debugger.on_user_action(UserAction::BreakpointSet {
        fragment: a.clone(),
    })?;
    debugger.run_program();
    assert_eq!(
        debugger
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .expect_leaf("main")
            .active_expression()
            .data
            .location,
        a,
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
            .active_expression()
            .data
            .location,
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
            .find(|fragment| fragment.data.location == a)
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
            .active_expression()
            .data
            .location,
        c,
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
            main: fn \ size_x, size_y ->
                1 2 f
            end

            # Add some arguments. In case the compiler decides to generate code
            # to handle those, this makes sure we step over that generated code.
            f: fn
                \ 1, a ->
                    nop # a

                \ 2, b ->
                    nop # b
            end
        ",
    );

    let (f, a) = {
        let named_functions = debugger.expect_code();

        let f = named_functions
            .find_by_name("main")
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body()
            .nth(2)
            .unwrap()
            .into_location();
        let a = named_functions
            .find_by_name("f")
            .unwrap()
            .branches()
            .next()
            .unwrap()
            .body()
            .next()
            .unwrap()
            .into_location();

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
            .active_expression()
            .data
            .location,
        a,
    );
}

#[test]
fn step_out_of_function_if_at_last_fragment() {
    // When stopping at the last fragment in a function and then stepping, we
    // expect to land at the fragment after the function call.

    let mut debugger = debugger();
    debugger.provide_source_code(
        r"
            main: fn \ size_x, size_y ->
                f
                nop
            end

            f: fn \ ->
                nop
                # There's a return instruction at the end of the function, which
                # we expect to step over.
            end
        ",
    );

    let (nop_in_main, nop_in_f) = {
        let named_functions = debugger.expect_code();

        let nop_in_main = named_functions
            .find_by_name("main")
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body()
            .nth(1)
            .unwrap()
            .into_location();
        let nop_in_f = named_functions
            .find_by_name("f")
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body()
            .next()
            .unwrap()
            .into_location();

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
            .active_expression()
            .data
            .location,
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
            main: fn \ size_x, size_y ->
                nop
            end
        ",
    );

    let nop = {
        let named_functions = debugger.expect_code();

        named_functions
            .find_by_name("main")
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body()
            .next()
            .unwrap()
            .into_location()
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
            main: fn \ size_x, size_y ->
                f
                nop
            end

            f: fn \ ->
                nop
            end
        ",
    );

    let [f, nop] = {
        let named_functions = debugger.expect_code();
        let mut body = named_functions
            .find_by_name("main")
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body();

        array::from_fn(|_| {
            body.find_map(|fragment| {
                if fragment.as_comment().is_none() {
                    Some(fragment.into_location())
                } else {
                    None
                }
            })
            .unwrap()
        })
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
            .active_expression()
            .data
            .location,
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
            main: fn \ size_x, size_y ->
                f
                nop # b
            end

            f: fn \ ->
                nop # a
                nop
            end
        ",
    );

    let (a, b) = {
        let named_functions = debugger.expect_code();

        let a = named_functions
            .find_by_name("f")
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body()
            .next()
            .unwrap()
            .into_location();
        let b = named_functions
            .find_by_name("main")
            .unwrap()
            .find_single_branch()
            .unwrap()
            .body()
            .nth(1)
            .unwrap()
            .into_location();

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
            .active_expression()
            .data
            .location,
        b,
    );
}
