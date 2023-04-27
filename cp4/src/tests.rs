use crate::{cp, test_report::TestReport};

pub fn run(functions: &mut cp::Functions) -> anyhow::Result<Vec<TestReport>> {
    let code = r#"
    "#;

    let data_stack = cp::execute(code.chars())?;
    if !data_stack.is_empty() {
        anyhow::bail!("Importing tests left values on stack: {data_stack:?}")
    }

    let mut results = Vec::new();

    for (name, function) in functions.tests() {
        let mut call_stack = cp::CallStack;
        let mut data_stack = cp::DataStack::new();

        let result = cp::evaluate(
            function.body,
            functions,
            &mut call_stack,
            &mut data_stack,
        );

        results.push(TestReport::new(
            function.module,
            name,
            result,
            data_stack,
        ));
    }

    results.sort_by_key(|report| report.result.is_ok());
    results.reverse();

    Ok(results)
}
