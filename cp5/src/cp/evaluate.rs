use super::{
    data_stack::{Array, Value},
    pipeline::ir::analyzer_output::{AnalyzerOutput, Expression},
    Bindings, DataStack, DataStackError, Function, FunctionBody, Functions,
    PipelineError,
};

pub struct Evaluator<'r> {
    pub data_stack: &'r mut DataStack,
    pub bindings: &'r mut Bindings,
    pub functions: &'r Functions,
    pub tests: &'r Functions,
}

impl Evaluator<'_> {
    pub fn evaluate_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<(), EvaluatorError> {
        match expression {
            Expression::Array { expressions } => {
                self.data_stack.mark();

                for expression in &expressions.elements {
                    self.evaluate_expression(expression)?;
                }

                let array = Array {
                    elements: self
                        .data_stack
                        .drain_values_from_marker()
                        .collect(),
                };
                let array = Value::Array(array);
                self.data_stack.push(array);
            }
            Expression::Binding { idents } => {
                for ident in idents.iter().rev() {
                    let value = self.data_stack.pop_any()?;
                    self.bindings.define(ident.clone(), value);
                }
            }
            Expression::EvalBinding { name } => {
                let value = self
                    .bindings
                    .get(name)
                    // This is a bug in the analyzer.
                    .expect("Binding eval must refer to binding");
                self.data_stack.push(value.clone());
            }
            Expression::EvalFunction { name } => {
                let function = self
                    .functions
                    .get(name)
                    // This is a bug in the analyzer.
                    .expect("Function eval must refer to function");
                self.evaluate_function(function)?;
            }
            Expression::Module { body, .. } => {
                for expression in &body.elements {
                    self.evaluate_expression(expression)?;
                }
            }
            Expression::Value(value) => {
                self.data_stack.push(value.clone());
            }
        }

        Ok(())
    }

    pub fn evaluate_expressions(
        &mut self,
        expressions: &AnalyzerOutput,
    ) -> Result<(), EvaluatorError> {
        for expression in &expressions.elements {
            self.evaluate_expression(expression)?;
        }

        Ok(())
    }

    pub fn evaluate_function(
        &mut self,
        function: &Function,
    ) -> Result<(), EvaluatorError> {
        match &function.body {
            FunctionBody::Intrinsic(intrinsic) => {
                intrinsic(self)?;
            }
            FunctionBody::UserDefined(expressions) => {
                self.evaluate_expressions(expressions)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {
    #[error(transparent)]
    DataStack(#[from] DataStackError),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}

impl From<DataStackError> for PipelineError<EvaluatorError> {
    fn from(err: DataStackError) -> Self {
        PipelineError::Stage(err.into())
    }
}
