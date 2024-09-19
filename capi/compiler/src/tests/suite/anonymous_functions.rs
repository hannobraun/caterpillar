use crate::tests::infra::compile_and_run;

#[test]
fn anonymous_function_eval() {
    let source = r"
        main: { ||
            { || 0 send }
                eval
        }
    ";

    compile_and_run(source);
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

    compile_and_run(source);
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

    compile_and_run(source);
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

    compile_and_run(source);
}
