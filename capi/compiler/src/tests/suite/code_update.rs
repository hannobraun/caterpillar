use crate::tests::infra::runtime;

#[test]
fn replace_single_instruction() {
    // When the new code only replaces a single instruction in the old one, we
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
