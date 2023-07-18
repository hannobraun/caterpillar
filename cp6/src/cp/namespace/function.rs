use crate::cp::{
    pipeline::ir::analyzer_output::AnalyzerOutput, Evaluator, EvaluatorError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    pub module: String,
    pub name: String,
    pub body: FunctionBody,
    pub is_test: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FunctionBody {
    Intrinsic(Intrinsic),
    UserDefined(AnalyzerOutput),
}

pub type Intrinsic = fn(&mut Evaluator) -> Result<(), EvaluatorError>;
