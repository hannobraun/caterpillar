use crate::tests::infra::runtime;

#[test]
#[should_panic] // https://github.com/hannobraun/caterpillar/issues/50
fn use_updated_code_on_next_recursive_function_call() {
    // If a function is updated, we expect the next call to it to execute the
    // new version.
    //
    // This test covers recursive calls, which is the easier test case to write.

    let mut runtime = runtime();

    runtime
        .update_code(
            r"
                main: { \ ->
                    0 send
                    main
                }
            ",
        )
        .run_until_receiving(0);

    runtime
        .update_code(
            r"
                main: { \ ->
                    1 send
                    main
                }
            ",
        )
        .run_until_receiving(1);
}

#[test]
fn use_old_code_before_next_function_call() {
    // If a function is updated while it's running, we expect it to still
    // execute the old code, until the next call to it.
    //
    // It would be better to have more fine-grained updates, but this behavior
    // is more practical, as it's relatively easy to achieve without a code
    // database and a custom editor.

    let mut runtime = runtime();

    runtime
        .update_code(
            r"
                main: { \ ->
                    0 send
                    1 send
                    main
                }
            ",
        )
        .run_until_receiving(0);

    runtime
        .update_code(
            r"
                main: { \ ->
                    0 send
                    2 send
                    main
                }
            ",
        )
        .run_until_receiving(1);
}

#[test]
#[should_panic] // https://github.com/hannobraun/caterpillar/issues/50
fn handle_update_that_makes_function_larger() {
    // The update procedure laid out by previous tests should still work, if the
    // update makes the function larger.

    let mut runtime = runtime();

    runtime
        .update_code(
            r"
                main: { \ ->
                    0 send
                    main
                }
            ",
        )
        .run_until_receiving(0);

    runtime
        .update_code(
            r"
                main: { \ ->
                    1 send
                    2 send
                    main
                }
            ",
        )
        .run_until_receiving(1)
        .run_until_receiving(2);
}

#[test]
#[should_panic] // https://github.com/hannobraun/caterpillar/issues/50
fn handle_update_that_makes_function_smaller() {
    // The update procedure laid out by previous tests should still work, if the
    // update makes the function smaller.

    let mut runtime = runtime();

    runtime
        .update_code(
            r"
                main: { \ ->
                    0 send
                    1 send
                    main
                }
            ",
        )
        .run_until_receiving(0);

    runtime
        .update_code(
            r"
                main: { \ ->
                    2 send
                    main
                }
            ",
        )
        .run_until_receiving(1)
        .run_until_receiving(2);
}
