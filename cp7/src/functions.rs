use std::{collections::BTreeMap, thread, time::Duration};

use crate::{
    data_stack::{DataStack, DataStackResult},
    value,
};

pub struct Functions {
    inner: BTreeMap<String, Function>,
}

impl Functions {
    pub fn new() -> Self {
        let mut inner = BTreeMap::new();

        let intrinsics = [
            ("+", add as Intrinsic),
            ("clone", clone),
            ("delay_ms", delay_ms),
            ("print_line", print_line),
            ("fn", fn_),
        ];

        for (name, intrinsic) in intrinsics {
            inner.insert(name.into(), Function::Intrinsic(intrinsic));
        }

        Self { inner }
    }

    pub fn define(&mut self, name: value::Symbol, body: value::Block) {
        let function = Function::UserDefined { body };
        self.inner.insert(name.0, function);
    }

    pub fn resolve(&self, name: &str) -> Result<&Function, ResolveError> {
        self.inner
            .get(name)
            .ok_or(ResolveError { name: name.into() })
    }
}

pub enum Function {
    Intrinsic(Intrinsic),
    UserDefined { body: value::Block },
}

pub type Intrinsic = fn(&mut Functions, &mut DataStack) -> DataStackResult<()>;

#[derive(Debug, thiserror::Error)]
#[error("Error resolving function `{name}`")]
pub struct ResolveError {
    pub name: String,
}

fn add(_: &mut Functions, data_stack: &mut DataStack) -> DataStackResult<()> {
    let b = data_stack.pop_specific::<value::Number>()?;
    let a = data_stack.pop_specific::<value::Number>()?;

    data_stack.push(value::Number(a.0 + b.0));

    Ok(())
}

fn clone(_: &mut Functions, data_stack: &mut DataStack) -> DataStackResult<()> {
    let value = data_stack.pop_any()?;

    data_stack.push(value.clone());
    data_stack.push(value);

    Ok(())
}

fn delay_ms(
    _: &mut Functions,
    data_stack: &mut DataStack,
) -> DataStackResult<()> {
    let delay_ms = data_stack.pop_specific::<value::Number>()?;
    thread::sleep(Duration::from_millis(delay_ms.0.try_into().unwrap()));
    Ok(())
}

fn fn_(
    functions: &mut Functions,
    data_stack: &mut DataStack,
) -> DataStackResult<()> {
    let body = data_stack.pop_specific::<value::Block>()?;
    let name = data_stack.pop_specific::<value::Symbol>()?;

    functions.define(name, body);

    Ok(())
}

fn print_line(
    _: &mut Functions,
    data_stack: &mut DataStack,
) -> DataStackResult<()> {
    let value = data_stack.pop_any()?;
    println!("{value}");
    Ok(())
}
