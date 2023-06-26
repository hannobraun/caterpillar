use crate::cp::{expressions::Expressions, Evaluator, EvaluatorError};

#[derive(Clone, Debug)]
pub struct Function {
    pub module: String,
    pub body: FunctionBody,
}

#[derive(Clone, Debug)]
pub enum FunctionBody {
    Intrinsic { body: IntrinsicBody },
    UserDefined { body: Expressions },
}

pub type IntrinsicBody = fn(&mut Evaluator) -> Result<(), EvaluatorError>;
