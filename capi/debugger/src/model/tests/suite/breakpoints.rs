use crate::model::{tests::infra::debugger, UserAction};

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
