use crate::{
    cp,
    test_report::{Error, TestReport},
};

pub fn run() -> anyhow::Result<Vec<TestReport>> {
    let code = r#"
        mod bool {
            test "true" { true }
            test "false not" { false not }
        }

        mod block {
            test "eval" { { true } eval }
        }

        mod fn_ {
            test "fn" { fn f { true } f }
        }

        mod string {
            test "=" { "a" "a" = }
        }
    "#;

    let mut data_stack = cp::DataStack::new();
    let mut functions = cp::Functions::new();
    let mut tests = cp::Functions::new();

    cp::execute(code, &mut data_stack, &mut functions, &mut tests)?;

    if !data_stack.is_empty() {
        anyhow::bail!("Importing tests left values on stack: {data_stack:?}")
    }

    let mut results = Vec::new();

    for (name, function) in tests {
        let mut syntax_elements = cp::StageInput::from(function.body);

        let mut data_stack = cp::DataStack::new();
        let mut functions = cp::Functions::new();
        let mut tests = cp::Functions::new();

        let mut end_result = Ok(());
        while !syntax_elements.is_empty() {
            let result = cp::evaluate(
                syntax_elements.reader(),
                &mut data_stack,
                &mut functions,
                &mut tests,
            );

            end_result = end_result.and(result);
        }

        let result = end_result
            .map_err(Error::Evaluator)
            .and_then(|()| {
                let test_passed = data_stack.pop_bool()?;
                if test_passed {
                    Ok(())
                } else {
                    Err(Error::TestFailed)
                }
            })
            .and_then(|()| {
                if data_stack.is_empty() {
                    Ok(())
                } else {
                    Err(Error::TestReturnedTooMuch)
                }
            });

        results.push(TestReport {
            module: function.module,
            name,
            result,
        })
    }

    Ok(results)
}
