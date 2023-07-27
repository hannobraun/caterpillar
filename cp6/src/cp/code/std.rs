use crate::cp;

pub fn define(
    functions: &mut cp::Functions,
    tests: &mut cp::Functions,
) -> anyhow::Result<()> {
    let code = r#"
        mod std {
            fn times {
                => block num .
                num 0 =
                    {}
                    {
                        block eval
                        num 1 - => num .
                        block num times
                    }
                        if
            }

            mod tests {
                test "times" { { true } 2 times drop }
            }
        }
    "#;

    let mut data_stack = cp::DataStack::new();
    let mut bindings = cp::Bindings::new();

    cp::execute(code, &mut data_stack, &mut bindings, functions, tests)?;

    if !data_stack.is_empty() {
        anyhow::bail!("Defining `std` left values on stack: {data_stack:?}")
    }

    Ok(())
}
