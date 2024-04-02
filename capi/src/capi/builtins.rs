#![allow(unused)]

use super::data_stack::DataStack;

pub fn add(data_stack: &mut DataStack) {
    let b = data_stack.pop();
    let a = data_stack.pop();

    let c = a + b;

    data_stack.push(c);
}

pub fn clone2(data_stack: &mut DataStack) {
    let b = data_stack.pop();
    let a = data_stack.pop();

    data_stack.push(a);
    data_stack.push(b);
    data_stack.push(a);
    data_stack.push(b);
}

pub fn mul(data_stack: &mut DataStack) {
    let b = data_stack.pop();
    let a = data_stack.pop();

    let c = a * b;

    data_stack.push(c);
}

pub fn store(data_stack: &mut DataStack, mem: &mut [u8]) {
    let value = data_stack.pop();
    let addr = data_stack.pop();

    let value: u8 = value.try_into().unwrap();
    mem[addr] = value;

    data_stack.push(addr);
}

pub fn swap(data_stack: &mut DataStack) {
    let b = data_stack.pop();
    let a = data_stack.pop();

    data_stack.push(b);
    data_stack.push(a);
}
