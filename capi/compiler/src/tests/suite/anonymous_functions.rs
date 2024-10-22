use crate::tests::infra::runtime;

#[test]
fn anonymous_function_eval() {
    runtime()
        .update_code(
            r"
                main: fn
                    \ ->
                        fn \ -> 0 send }
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
                main: fn
                    \ ->
                        0
                        fn \ channel -> channel }
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
                main: fn
                    \ ->
                        0
                        fn
                            \ channel ->
                                channel
                                fn \ channel -> channel }
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
                main: fn
                    \ ->
                        0
                        fn
                            \ channel ->
                                fn \ ->
                                    # We are not using `channel` here, to make
                                    # sure that capturing works even from a
                                    # grandparent scope.

                                    fn \ -> channel send }
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
