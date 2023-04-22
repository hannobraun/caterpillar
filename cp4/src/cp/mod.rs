use std::iter;

pub struct Functions {}

impl Functions {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tests(&self) -> impl Iterator<Item = (String, Function)> {
        iter::empty()
    }
}

pub struct Function {
    pub module: String,
    pub body: String,
}

pub struct CallStack;

#[derive(Debug)]
pub struct DataStack {}

impl DataStack {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn pop_bool(&mut self) -> Result<bool, DataStackError> {
        Ok(false)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataStackError {}

pub fn execute(
    _: impl IntoIterator<Item = char>,
    _: &mut Functions,
) -> anyhow::Result<DataStack> {
    Ok(DataStack {})
}

pub fn evaluate(
    _: String,
    _: &mut Functions,
    _: &mut CallStack,
    _: &mut DataStack,
) -> Result<(), EvaluatorError> {
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluatorError {}
