use std::collections::BTreeMap;

use crate::cp;

pub struct TestReport {
    pub name: &'static str,
    pub result: Result<(), Error>,
}

pub fn run() -> Vec<TestReport> {
    let mut tests = BTreeMap::new();

    tests.insert("true", "true");
    tests.insert("false not", "false not");
    tests.insert("drop", "true false drop");
    tests.insert("clone", "true clone drop");
    tests.insert("binding", "true false => t f . t");
    tests.insert("block eval", "{ true } eval");
    tests.insert("block - lazy evaluation", "true { drop } drop");
    tests.insert("array - eager evaluation", "true false [ drop ] drop");

    let mut results = Vec::new();

    for (name, code) in tests {
        let result = cp::execute(code)
            .map_err(Error::Language)
            .and_then(|mut data_stack| {
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

    results
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Language(cp::Error),

    #[error(transparent)]
    ReturnValue(#[from] cp::DataStackError),

    #[error("Test did not return `true`")]
    TestFailed,

    #[error("Test returned too many values")]
    TestReturnedTooMuch,
}
