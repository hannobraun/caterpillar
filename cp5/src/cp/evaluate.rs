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

fn clone(evaluator: &mut Evaluator) -> Result<(), EvaluatorError> {
    let a = evaluator.data_stack.pop_any()?;

    evaluator.data_stack.push(a.clone());
    evaluator.data_stack.push(a);

    Ok(())
}

fn drop(evaluator: &mut Evaluator) -> Result<(), EvaluatorError> {
    evaluator.data_stack.pop_any()?;
    Ok(())
}

fn true_(evaluator: &mut Evaluator) -> Result<(), EvaluatorError> {
    evaluator.data_stack.push(true);
    Ok(())
}

fn false_(evaluator: &mut Evaluator) -> Result<(), EvaluatorError> {
    evaluator.data_stack.push(false);
    Ok(())
}

fn and(evaluator: &mut Evaluator) -> Result<(), EvaluatorError> {
    let a = evaluator.data_stack.pop_bool()?;
    let b = evaluator.data_stack.pop_bool()?;

    evaluator.data_stack.push(a && b);

    Ok(())
}

fn not(evaluator: &mut Evaluator) -> Result<(), EvaluatorError> {
    let b = evaluator.data_stack.pop_bool()?;
    evaluator.data_stack.push(!b);

    Ok(())
}

fn if_(evaluator: &mut Evaluator) -> Result<(), EvaluatorError> {
    let else_ = evaluator.data_stack.pop_block()?;
    let then_ = evaluator.data_stack.pop_block()?;
    let cond = evaluator.data_stack.pop_bool()?;

    let block = if cond { then_ } else { else_ };

    evaluator.evaluate_block(block)?;

    Ok(())
}

fn unwrap(evaluator: &mut Evaluator) -> Result<(), EvaluatorError> {
    let array = evaluator.data_stack.pop_array()?;

    for value in array.elements {
        evaluator.data_stack.push(value);
    }

    Ok(())
}

fn eval(evaluator: &mut Evaluator) -> Result<(), EvaluatorError> {
    let block = evaluator.data_stack.pop_block()?;
    evaluator.evaluate_block(block)?;
    Ok(())
}

fn eq(evaluator: &mut Evaluator) -> Result<(), EvaluatorError> {
    let b = evaluator.data_stack.pop_any()?;
    let a = evaluator.data_stack.pop_any()?;

    let eq = a == b;

    evaluator.data_stack.push(eq);

    Ok(())
}

fn sub(evaluator: &mut Evaluator) -> Result<(), EvaluatorError> {
    let b = evaluator.data_stack.pop_u8()?;
    let a = evaluator.data_stack.pop_u8()?;

    evaluator.data_stack.push(a - b);

    Ok(())
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
