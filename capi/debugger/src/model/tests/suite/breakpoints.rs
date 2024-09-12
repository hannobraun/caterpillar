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
        .run_process();

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

    debugger.run_process();

    assert_eq!(
        debugger
            .state
            .generate_transient_state()
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
