use crate::tests::infra::runtime;

#[test]
fn anonymous_function_eval() {
    let source = r"
        main: { ||
            { || 0 send }
                eval
        }
    ";

    runtime().update_code(source).run_until_receiving(0);
}

#[test]
fn anonymous_function_parameter() {
    let source = r"
        main: { ||
            0
            { |channel| channel }
                eval
                send
        }
    ";

    runtime().update_code(source).run_until_receiving(0);
}

#[test]
fn anonymous_function_parameter_shadowing() {
    let source = r"
        main: { ||
            0
            { |channel|
                channel
                { |channel| channel }
                    eval
            }
                eval
                send
        }
    ";

    runtime().update_code(source).run_until_receiving(0);
}

#[test]
fn anonymous_function_captured_binding() {
    let source = r"
        main: { ||
            0
            { |channel|
                { ||
                    # We are not using `channel` here, to make sure that
                    # capturing works even from a grandparent scope.

                    { || channel send }
                        eval
                }
                    eval
            }
                eval
        }
    ";

    runtime().update_code(source).run_until_receiving(0);
}
