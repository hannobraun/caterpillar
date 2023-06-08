use crate::{
    cp,
    test_report::{Error, TestReport},
};

pub fn run() -> anyhow::Result<Vec<TestReport>> {
    let code = r#"
        mod bool {
            test "true" { true }
            test "false not" { false not }
            test "and - true true" { true true and }
            test "and - true false" { true false and not }
            test "and - false true" { false true and not }
            test "and - false false" { false false and not }
        }

        mod binding {
            test "binding" { true false => t f . t }
        }

        mod basics {
            test "drop" { true false drop }
            test "clone" { true clone drop }
        }

        mod block {
            test "eval" { { true } eval }
            test "lazy evaluation" { true { drop } drop }
        }

        mod array {
            test "unwrap" { [ true ] unwrap }
            test "eager evaluation" { true false [ drop ] drop }
        }

        mod fn_ {
            test "fn" { fn f { true } f }
        }

        mod if_ {
            test "then" { true { true } { false } if }
            test "else" { false { false } { true } if }
        }

        mod string {
            test "=" { "a" "a" = }
            test "= not" { "a" "b" = not }
        }
    "#;

    let mut data_stack = cp::DataStack::new();
    let mut bindings = cp::Bindings::new();
    let mut functions = cp::Functions::new();
    let mut tests = cp::Functions::new();

    cp::execute(
        code,
        &mut data_stack,
        &mut bindings,
        &mut functions,
        &mut tests,
    )?;

    if !data_stack.is_empty() {
        anyhow::bail!("Importing tests left values on stack: {data_stack:?}")
    }

    let mut results = Vec::new();

    for (name, function) in tests {
        let syntax_elements = cp::StageInput::from(function.body);

        let mut data_stack = cp::DataStack::new();
        let mut bindings = cp::Bindings::new();
        let mut functions = cp::Functions::new();
        let mut tests = cp::Functions::new();

        let result = cp::evaluate_all(
            syntax_elements,
            &mut data_stack,
            &mut bindings,
            &mut functions,
            &mut tests,
        );

        let result = result
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
