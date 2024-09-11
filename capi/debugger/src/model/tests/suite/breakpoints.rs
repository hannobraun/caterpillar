use crate::model::{
    tests::infra::{
        debugger, ActiveFunctionsEntriesExt, ActiveFunctionsExt, BranchExt,
        DebugFunctionExt, FunctionsExt, TestDebugger,
    },
    DebugFragment,
};

#[test]
fn display_breakpoint_that_was_set() -> anyhow::Result<()> {
    // Breakpoints that are set in the debugger state should be displayed.

    let mut debugger = debugger()
        .provide_source_code(
            r"
                main: { |size_x size_y|
                    nop
                }
            ",
        )
        .run_process();
    fn fragment(debugger: &TestDebugger) -> DebugFragment {
        debugger
            .state
            .generate_transient_state()
            .active_functions
            .expect_entries()
            .functions()
            .with_name("main")
            .only_branch()
            .fragment(0)
    }

    assert!(!fragment(&debugger).data.has_durable_breakpoint);

    debugger
        .state
        .set_durable_breakpoint(&fragment(&debugger).data.id)?;
    assert!(fragment(&debugger).data.has_durable_breakpoint);

    Ok(())
}
