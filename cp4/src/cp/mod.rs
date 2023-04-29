mod data_stack;

pub use self::data_stack::{DataStack, DataStackError};

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}

pub fn execute(code: &str) -> (Result<(), EvaluatorError>, DataStack) {
    let mut data_stack = DataStack::new();

    for code in code.split_whitespace() {
        let value = match code {
            "true" => true,
            "false" => false,
            word => {
                return (
                    Err(EvaluatorError::UnknownWord(word.into())),
                    data_stack,
                )
            }
        };

        data_stack.push(value);
    }

    (Ok(()), data_stack)
}
