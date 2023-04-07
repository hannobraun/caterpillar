use std::collections::BTreeMap;

use crate::cp;

pub struct TestReport {
    pub name: &'static str,
    pub result: Result<(), Error>,
}

pub fn run() -> Vec<TestReport> {
    let mut tests = BTreeMap::new();

    tests.insert("true", r#"true"#);
    tests.insert("false not", r#"false not"#);
    tests.insert("and - true true", r#"true true and"#);
    tests.insert("and - true false", r#"true false and not"#);
    tests.insert("and - false true", r#"false true and not"#);
    tests.insert("and - false false", r#"false false and not"#);
    tests.insert("drop", r#"true false drop"#);
    tests.insert("clone", r#"true clone drop"#);
    tests.insert("binding", r#"true false => t f . t"#);
    tests.insert("block eval", r#"{ true } eval"#);
    tests.insert("block - lazy evaluation", r#"true { drop } drop"#);
    tests.insert("array unwrap", r#"[ true ] unwrap"#);
    tests.insert("array - eager evaluation", r#"true false [ drop ] drop"#);
    tests.insert("fn", r#"fn f { true } f"#);
    tests.insert("if then", r#"true { true } { false } if"#);
    tests.insert("if else", r#"false { false } { true } if"#);
    tests.insert("flexible tokenization", r#"{true}eval[true]unwrap and"#);

    let mut results = Vec::new();

    for (name, code) in tests {
        let result = cp::execute(code.chars())
            .map_err(Error::Language)
            .and_then(|(_, mut data_stack)| {
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
