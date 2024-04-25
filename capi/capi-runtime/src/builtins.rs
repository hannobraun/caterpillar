#![allow(unused)]

use super::data_stack::DataStack;

pub fn add(data_stack: &mut DataStack) -> Result {
    let b = data_stack.pop();
    let a = data_stack.pop();

    let c = a + b;

    data_stack.push(c);

    Ok(())
}

pub fn copy(data_stack: &mut DataStack) -> Result {
    let mut i = data_stack.pop();

    data_stack.save(i);
    let a = data_stack.clone();
    data_stack.restore();

    data_stack.push(a);

    Ok(())
}

pub fn drop(data_stack: &mut DataStack) -> Result {
    let i = data_stack.pop();

    data_stack.save(i);
    data_stack.pop();
    data_stack.restore();

    Ok(())
}

pub fn mul(data_stack: &mut DataStack) -> Result {
    let b = data_stack.pop();
    let a = data_stack.pop();

    let c = a * b;

    data_stack.push(c);

    Ok(())
}

pub fn place(data_stack: &mut DataStack) -> Result {
    let mut i = data_stack.pop();
    let mut a = data_stack.pop();

    data_stack.save(i);
    data_stack.push(a);
    data_stack.restore();

    Ok(())
}

pub fn store(data_stack: &mut DataStack, mem: &mut [u8]) -> Result {
    let value = data_stack.pop();
    let addr = data_stack.pop();

    let value: u8 = value.try_into().unwrap();
    mem[addr] = value;

    data_stack.push(addr);

    Ok(())
}

pub fn sub(data_stack: &mut DataStack) -> Result {
    let b = data_stack.pop();
    let a = data_stack.pop();

    let c = a.wrapping_sub(b);

    data_stack.push(c);

    Ok(())
}

pub fn swap(data_stack: &mut DataStack) -> Result {
    let b = data_stack.pop();
    let a = data_stack.pop();

    data_stack.push(b);
    data_stack.push(a);

    Ok(())
}

pub fn take(data_stack: &mut DataStack) -> Result {
    let mut i = data_stack.pop();

    data_stack.save(i);
    let a = data_stack.pop();
    data_stack.restore();

    data_stack.push(a);

    Ok(())
}

pub type Result = std::result::Result<(), Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {}
