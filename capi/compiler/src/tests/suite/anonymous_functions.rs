use crate::tests::infra::compile_and_run;

#[test]
fn anonymous_function_eval() {
    let source = r"
        main: { ||
            { || 0 send }
                eval
        }
    ";

    let mut signals = compile_and_run(source);

    assert_eq!(signals.remove(&0), Some(1));
    assert!(signals.is_empty());
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

    let mut signals = compile_and_run(source);

    assert_eq!(signals.remove(&0), Some(1));
    assert!(signals.is_empty());
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

    let mut signals = compile_and_run(source);

    assert_eq!(signals.remove(&0), Some(1));
    assert!(signals.is_empty());
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

    let mut signals = compile_and_run(source);

    assert_eq!(signals.remove(&0), Some(1));
    assert!(signals.is_empty());
}
