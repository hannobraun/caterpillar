use crate::tests::infra::runtime;

#[test]
fn replace_single_instruction() {
    // If the new code only replaces a single instruction in the old one, we
    // expect the new instruction to be used from then on.

    let mut runtime = runtime();

    runtime
        .update_code(
            r"
                main: { ||
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
                main: { ||
                    0 send
                    2 send
                    main
                }
            ",
        )
        .run_until_receiving(2);
}

#[test]
fn replace_block_of_instructions() {
    // If the new code replaces a block of multiple neighboring instructions, we
    // expect the new instructions to be used from then on.

    let mut runtime = runtime();

    runtime
        .update_code(
            r"
                main: { ||
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
                main: { ||
                    1 send
                    2 send
                    main
                }
            ",
        )
        .run_until_receiving(2);
}

#[test]
fn add_additional_instructions_after_current_instruction() {
    // If the new code adds new instructions after the current instruction, we
    // expect those to be executed next.

    let mut runtime = runtime();

    runtime
        .update_code(
            r"
                main: { ||
                    0 send
                    main
                }
            ",
        )
        .run_until_receiving(0);

    runtime
        .update_code(
            r"
                main: { ||
                    0 send
                    1 send
                    main
                }
            ",
        )
        .run_until_receiving(1);
}

#[test]
#[should_panic] // https://github.com/hannobraun/caterpillar/issues/50
fn add_additional_instructions_before_current_instruction() {
    // If the new code adds new instructions before the current instruction, we
    // expect those to not disturb the flow of execution.

    let mut runtime = runtime();

    runtime
        .update_code(
            r"
                main: { ||
                    1 send
                    2 send
                    main
                }
            ",
        )
        .run_until_receiving(1);

    runtime
        .update_code(
            r"
                main: { ||
                    0 send
                    1 send
                    2 send
                    main
                }
            ",
        )
        .run_until_receiving(2);
}
