use crate::data_stack::{value, DataStack, DataStackResult};

pub fn add(data_stack: &mut DataStack) -> DataStackResult<()> {
    let b = data_stack.pop_number()?;
    let a = data_stack.pop_number()?;

    data_stack.push(value::Number(a.0 + b.0));

    Ok(())
}

pub fn print_line(data_stack: &mut DataStack) -> DataStackResult<()> {
    let value = data_stack.pop_any()?;
    println!("{value}");
    Ok(())
}
