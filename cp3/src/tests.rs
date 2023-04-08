use crate::cp;

pub struct TestReport {
    pub name: String,
    pub result: Result<(), Error>,
}

pub fn run() -> anyhow::Result<Vec<TestReport>> {
    let code = r#"
        test "true" { true }
        test "false not" { false not }
        test "and - true true" { true true and }
        test "and - true false" { true false and not }
        test "and - false true" { false true and not }
        test "and - false false" { false false and not }
        test "drop" { true false drop }
        test "clone" { true clone drop }
        test "binding" { true false => t f . t }
        test "block eval" { { true } eval }
        test "block - lazy evaluation" { true { drop } drop }
        test "block - tokenization" { {true}eval{true}eval and }
        test "array unwrap" { [ true ] unwrap }
        test "array - eager evaluation" { true false [ drop ] drop }
        test "array - tokenization" { [true]unwrap[true]unwrap and }
        test "fn" { fn f { true } f }
        test "if then" { true { true } { false } if }
        test "if else" { false { false } { true } if }
        test "string =" { "a" "a" = }
        test "string = not" { "a" "b" = not }
        test "string - tokenization" { "a""a"="b""b"= and }
    "#;

    let (functions, _) = cp::execute(code.chars())?;

    let mut results = Vec::new();

    for (name, code) in functions.tests() {
        let mut call_stack = cp::CallStack;
        let mut data_stack = cp::DataStack::new();

        let result = cp::evaluate(
            code.body,
            &functions,
            &mut call_stack,
            &mut data_stack,
        )
        .map_err(Error::Language)
        .and_then(|()| {
            if data_stack.pop_bool()? {
                Ok(data_stack)
            } else {
                Err(Error::TestFailed)
            }
        })
        .and_then(|data_stack| {
            if data_stack.is_empty() {
                Ok(())
            } else {
                Err(Error::TestReturnedTooMuch)
            }
        });

        results.push(TestReport { name, result });
    }

    results.sort_by_key(|report| report.result.is_ok());
    results.reverse();

    Ok(results)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Language(cp::EvaluatorError),

    #[error(transparent)]
    ReturnValue(#[from] cp::DataStackError),

    #[error("Test did not return `true`")]
    TestFailed,

    #[error("Test returned too many values")]
    TestReturnedTooMuch,
}
