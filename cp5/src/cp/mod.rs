pub fn execute(code: &str) -> bool {
    code == "true"
}

#[derive(Debug, thiserror::Error)]
pub enum DataStackError {}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {}
