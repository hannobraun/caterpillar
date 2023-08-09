use std::{collections::BTreeMap, thread, time::Duration};

use super::{
    runtime::data_stack::{DataStack, DataStackResult},
    syntax::SyntaxHandle,
    value,
};

#[derive(Debug)]
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

    pub fn replace(&mut self, old: SyntaxHandle, new: SyntaxHandle) {
        for function in self.inner.values_mut() {
            if let Function::UserDefined { body } = function {
                if let Some(handle) = &mut body.0 {
                    if handle.hash == old.hash {
                        *handle = new;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
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
