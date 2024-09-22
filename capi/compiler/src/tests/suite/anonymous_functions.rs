use crate::tests::infra::runtime;

#[test]
fn anonymous_function_eval() {
    runtime()
        .update_code(
            r"
                main: { \ ->
                    { \ -> 0 send }
                        eval
                }
            ",
        )
        .run_until_receiving(0);
}

#[test]
fn anonymous_function_parameter() {
    runtime()
        .update_code(
            r"
                main: { \ ->
                    0
                    { \ channel -> channel }
                        eval
                        send
                }
            ",
        )
        .run_until_receiving(0);
}

#[test]
fn anonymous_function_parameter_shadowing() {
    runtime()
        .update_code(
            r"
                main: { \ ->
                    0
                    { \ channel ->
                        channel
                        { \ channel -> channel }
                            eval
                    }
                        eval
                        send
                }
            ",
        )
        .run_until_receiving(0);
}

#[test]
fn anonymous_function_captured_binding() {
    runtime()
        .update_code(
            r"
                main: { \ ->
                    0
                    { \ channel ->
                        { \ ->
                            # We are not using `channel` here, to make sure that
                            # capturing works even from a grandparent scope.

                            { \ -> channel send }
                                eval
                        }
                            eval
                    }
                        eval
                }
            ",
        )
        .run_until_receiving(0);
}
