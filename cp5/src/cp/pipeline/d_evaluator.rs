#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}
