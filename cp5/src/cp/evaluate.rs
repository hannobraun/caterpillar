use crate::intrinsics::{
    and, clone, drop, eq, eval, false_, if_, not, sub, true_, unwrap,
};

use super::{
    data_stack::{Array, Value},
    pipeline::c_analyzer::Expressions,
    syntax::SyntaxElement,
    Bindings, DataStack, DataStackError, Expression, Function, FunctionKind,
    Functions, PipelineError,
};

pub struct Evaluator<'r> {
    pub data_stack: &'r mut DataStack,
    pub bindings: &'r mut Bindings,
    pub functions: &'r Functions,
    pub tests: &'r Functions,
}

impl Evaluator<'_> {
    pub fn evaluate_block(
        &mut self,
        block: &Expressions,
    ) -> Result<(), EvaluatorError> {
        for expression in &block.elements {
            self.evaluate_expression(expression)?;
        }

        Ok(())
    }

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
                    self.bindings.inner.insert(ident.clone(), value);
                }
            }
            Expression::EvalFunction { name } => {
                self.evaluate_word(name)?;
            }
            Expression::Module { body, .. } => {
                for expression in &body.elements {
                    self.evaluate_expression(expression)?;
                }
            }
            Expression::Value(value) => {
                self.data_stack.push(value.clone());
            }
            Expression::RawSyntaxElement(SyntaxElement::Word(word)) => {
                self.evaluate_word(word)?;
            }
            Expression::RawSyntaxElement(syntax_element) => {
                panic!("Unexpected raw syntax element: {syntax_element:?}")
            }
        }

        Ok(())
    }

    pub fn evaluate_function(
        &mut self,
        function: &Function,
    ) -> Result<(), EvaluatorError> {
        let FunctionKind::UserDefined { body, .. } = &function.body;
        self.evaluate_block(body)?;
        Ok(())
    }

    pub fn evaluate_word(&mut self, word: &str) -> Result<(), EvaluatorError> {
        match word {
            "clone" => clone(self)?,
            "drop" => drop(self)?,
            "true" => true_(self)?,
            "false" => false_(self)?,
            "and" => and(self)?,
            "not" => not(self)?,
            "if" => if_(self)?,
            "unwrap" => unwrap(self)?,
            "eval" => eval(self)?,
            "=" => eq(self)?,
            "-" => sub(self)?,
            _ => {
                if let Some(value) = self.bindings.inner.get(word) {
                    self.data_stack.push(value.clone());
                    return Ok(());
                }

                if let Some(function) = self.functions.get(word) {
                    self.evaluate_function(&function)?;
                    return Ok(());
                }

                if let Ok(value) = word.parse::<u8>() {
                    self.data_stack.push(value);
                    return Ok(());
                }

                return Err(EvaluatorError::UnknownWord(word.into()));
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
