use std::collections::BTreeMap;

use crate::data_stack::{value, DataStack, DataStackResult};

pub struct Functions {
    inner: BTreeMap<&'static str, Function>,
}

impl Functions {
    pub fn new() -> Self {
        let mut inner = BTreeMap::new();

        inner.insert("+", Function::Intrinsic(add));
        inner.insert("print_line", Function::Intrinsic(print_line));

        Self { inner }
    }

    pub fn resolve(&self, name: &str) -> Result<&Function, ResolveError> {
        self.inner
            .get(name)
            .ok_or(ResolveError { name: name.into() })
    }
}

pub enum Function {
    Intrinsic(Intrinsic),
}

pub type Intrinsic = fn(&mut DataStack) -> DataStackResult<()>;

#[derive(Debug, thiserror::Error)]
#[error("Error resolving function `{name}`")]
pub struct ResolveError {
    pub name: String,
}

fn add(data_stack: &mut DataStack) -> DataStackResult<()> {
    let b = data_stack.pop_number()?;
    let a = data_stack.pop_number()?;

    data_stack.push(value::Number(a.0 + b.0));

    Ok(())
}

fn print_line(data_stack: &mut DataStack) -> DataStackResult<()> {
    let value = data_stack.pop_any()?;
    println!("{value}");
    Ok(())
}
