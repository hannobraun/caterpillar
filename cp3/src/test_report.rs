use crate::cp;

pub struct TestReport {
    pub module: String,
    pub name: String,
    pub result: Result<(), Error>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Evaluator(cp::EvaluatorError),

    #[error(transparent)]
    ReturnValue(#[from] cp::DataStackError),

    #[error("Test did not return `true`")]
    TestFailed,

    #[error("Test returned too many values")]
    TestReturnedTooMuch,
}
