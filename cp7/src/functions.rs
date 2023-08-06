use std::collections::BTreeMap;

use crate::data_stack::{value, DataStack, DataStackResult};

pub struct Functions {
    inner: BTreeMap<&'static str, Intrinsic>,
}

impl Functions {
    pub fn new() -> Self {
        let mut inner = BTreeMap::new();

        inner.insert("+", add as Intrinsic);
        inner.insert("print_line", print_line);

        Self { inner }
    }

    pub fn resolve(&self, name: &str) -> Option<Intrinsic> {
        self.inner.get(name).copied()
    }
}

pub type Intrinsic = fn(&mut DataStack) -> DataStackResult<()>;

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
