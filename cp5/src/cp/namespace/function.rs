use crate::cp::{expressions::Expressions, Evaluator, EvaluatorError};

#[derive(Clone, Debug)]
pub struct Function {
    pub module: String,
    pub body: FunctionBody,
}

#[derive(Clone, Debug)]
pub enum FunctionBody {
    Intrinsic(IntrinsicBody),
    UserDefined(Expressions),
}

pub type IntrinsicBody = fn(&mut Evaluator) -> Result<(), EvaluatorError>;
