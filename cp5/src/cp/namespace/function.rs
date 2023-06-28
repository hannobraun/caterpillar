use crate::cp::{
    pipeline::ir::analyzer_output::AnalyzerOutput, Evaluator, EvaluatorError,
};

#[derive(Clone, Debug)]
pub struct Function {
    pub module: String,
    pub body: FunctionBody,
}

#[derive(Clone, Debug)]
pub enum FunctionBody {
    Intrinsic(Intrinsic),
    UserDefined(AnalyzerOutput),
}

pub type Intrinsic = fn(&mut Evaluator) -> Result<(), EvaluatorError>;
