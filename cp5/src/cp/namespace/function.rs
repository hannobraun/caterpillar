use crate::cp::{
    pipeline::ir::expressions::Expressions, Evaluator, EvaluatorError,
};

#[derive(Clone, Debug)]
pub struct Function {
    pub module: String,
    pub body: FunctionBody,
}

#[derive(Clone, Debug)]
pub enum FunctionBody {
    Intrinsic(Intrinsic),
    UserDefined(Expressions),
}

pub type Intrinsic = fn(&mut Evaluator) -> Result<(), EvaluatorError>;
