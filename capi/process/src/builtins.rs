use crate::{CoreEffect, Function, InstructionAddr, Instructions, Stack};

pub fn add(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let Some(c) = a.checked_add(b) else {
        return Err(CoreEffect::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(())
}

pub fn add_wrap_unsigned(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let c = a.wrapping_add(b);
    let c = if c >= 0 { c } else { c - i32::MIN };

    stack.push_operand(c);

    Ok(())
}

pub fn brk() -> Result {
    Err(CoreEffect::Breakpoint)
}

pub fn copy(stack: &mut Stack) -> Result {
    let a = stack.pop_operand()?;

    stack.push_operand(a);
    stack.push_operand(a);

    Ok(())
}

pub fn div(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    if b == 0 {
        return Err(CoreEffect::DivideByZero);
    }
    let Some(c) = a.checked_div(b) else {
        // Can't be divide by zero. Already handled that.
        return Err(CoreEffect::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(())
}

pub fn drop(stack: &mut Stack) -> Result {
    stack.pop_operand()?;
    Ok(())
}

pub fn eq(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let c = if a.0 == b.0 { 1 } else { 0 };

    stack.push_operand(c);

    Ok(())
}

pub fn eval(stack: &mut Stack, instructions: &Instructions) -> Result {
    let block = stack.pop_operand()?;

    stack.push_frame(
        Function {
            arguments: Vec::new(),
            first_instruction: InstructionAddr {
                index: u32::from_le_bytes(block.0),
            },
        },
        instructions,
    )?;

    Ok(())
}

pub fn greater(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let c = if a > b { 1 } else { 0 };

    stack.push_operand(c);

    Ok(())
}

pub fn if_(stack: &mut Stack, instructions: &Instructions) -> Result {
    let else_ = stack.pop_operand()?;
    let then = stack.pop_operand()?;
    let condition = stack.pop_operand()?;

    let block = if condition.0 == [0, 0, 0, 0] {
        else_
    } else {
        then
    };

    stack.push_frame(
        Function {
            arguments: Vec::new(),
            first_instruction: InstructionAddr {
                index: u32::from_le_bytes(block.0),
            },
        },
        instructions,
    )?;

    Ok(())
}

pub fn mul(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let Some(c) = a.checked_mul(b) else {
        return Err(CoreEffect::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(())
}

pub fn neg(stack: &mut Stack) -> Result {
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);

    if a == i32::MIN {
        return Err(CoreEffect::IntegerOverflow);
    }
    let b = -a;

    stack.push_operand(b);

    Ok(())
}

pub fn remainder(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    if b == 0 {
        return Err(CoreEffect::DivideByZero);
    }
    let c = a % b;

    stack.push_operand(c);

    Ok(())
}

pub fn sub(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let Some(c) = a.checked_sub(b) else {
        return Err(CoreEffect::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(())
}

pub type Result = std::result::Result<(), CoreEffect>;
