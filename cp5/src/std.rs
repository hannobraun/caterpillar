use crate::cp;

pub fn define() -> anyhow::Result<cp::Functions> {
    let mut functions = cp::Functions::new();

    let code = r#"
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
    "#;

    let mut data_stack = cp::DataStack::new();
    let mut bindings = cp::Bindings::new();
    let mut tests = cp::Functions::new();

    cp::execute(
        code,
        &mut data_stack,
        &mut bindings,
        &mut functions,
        &mut tests,
    )?;

    if !data_stack.is_empty() {
        anyhow::bail!("Defining `std` left values on stack: {data_stack:?}")
    }

    Ok(functions)
}
