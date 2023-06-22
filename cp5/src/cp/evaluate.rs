use super::{
    data_stack::{Array, Value},
    pipeline::c_analyzer::Expressions,
    syntax::SyntaxElement,
    Bindings, DataStack, DataStackError, Expression, Function, Functions,
    PipelineError,
};

pub struct Evaluator<'r> {
    pub data_stack: &'r mut DataStack,
    pub bindings: &'r mut Bindings,
    pub functions: &'r Functions,
    pub tests: &'r Functions,
}

impl Evaluator<'_> {
    fn evaluate_block(
        &mut self,
        block: Expressions,
    ) -> Result<(), EvaluatorError> {
        for expression in block.elements {
            self.evaluate_expression(&expression)?;
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

    fn evaluate_word(&mut self, word: &str) -> Result<(), EvaluatorError> {
        match word {
            "clone" => {
                let a = self.data_stack.pop_any()?;

                self.data_stack.push(a.clone());
                self.data_stack.push(a);
            }
            "drop" => {
                self.data_stack.pop_any()?;
            }
            "true" => self.data_stack.push(true),
            "false" => self.data_stack.push(false),
            "and" => {
                let a = self.data_stack.pop_bool()?;
                let b = self.data_stack.pop_bool()?;

                self.data_stack.push(a && b);
            }
            "not" => {
                let b = self.data_stack.pop_bool()?;
                self.data_stack.push(!b);
            }
            "if" => {
                let else_ = self.data_stack.pop_block()?;
                let then_ = self.data_stack.pop_block()?;
                let cond = self.data_stack.pop_bool()?;

                let block = if cond { then_ } else { else_ };

                self.evaluate_block(block)?;
            }
            "unwrap" => {
                let array = self.data_stack.pop_array()?;

                for value in array.elements {
                    self.data_stack.push(value);
                }
            }
            "eval" => {
                let block = self.data_stack.pop_block()?;
                self.evaluate_block(block)?;
            }
            "=" => {
                let b = self.data_stack.pop_any()?;
                let a = self.data_stack.pop_any()?;

                let eq = a == b;

                self.data_stack.push(eq);
            }
            "-" => {
                let b = self.data_stack.pop_u8()?;
                let a = self.data_stack.pop_u8()?;

                self.data_stack.push(a - b);
            }
            _ => {
                if let Some(value) = self.bindings.inner.get(word) {
                    self.data_stack.push(value.clone());
                    return Ok(());
                }

                if let Some(function) = self.functions.get(word) {
                    let Function::UserDefined { body, .. } = function;
                    self.evaluate_block(body)?;
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
