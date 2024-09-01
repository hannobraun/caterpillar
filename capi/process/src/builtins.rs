use crate::{value::IntegerOverflow, Effect, Stack};

pub fn builtin_by_name(name: &str) -> Option<Builtin> {
    let builtin = match name {
        "drop" => drop,
        "eq" => eq,
        "greater_i8" => greater_i8,
        "greater_i32" => greater_i32,
        "greater_u8" => greater_u8,
        "i32_to_i8" => i32_to_i8,
        "mul_i32" => mul_i32,
        "mul_u8_wrap" => mul_u8_wrap,
        "neg_i32" => neg_i32,
        "not" => not,
        "remainder_i32" => remainder_i32,
        "sub_i32" => sub_i32,
        "sub_u8" => sub_u8,
        "sub_u8_wrap" => sub_u8_wrap,

        _ => {
            return None;
        }
    };

    Some(builtin)
}

pub type Builtin = fn(&mut Stack) -> Result;

fn drop(stack: &mut Stack) -> Result {
    stack.pop_operand()?;
    Ok(())
}

fn eq(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let c = if a.0 == b.0 { 1 } else { 0 };

    stack.push_operand(c);

    Ok(())
}

fn greater_i8(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = a.to_i8()?;
    let b = b.to_i8()?;

    let c = if a > b { 1 } else { 0 };

    stack.push_operand(c);

    Ok(())
}

fn greater_i32(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = a.to_i32();
    let b = b.to_i32();

    let c = if a > b { 1 } else { 0 };

    stack.push_operand(c);

    Ok(())
}

fn greater_u8(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = a.to_u8()?;
    let b = b.to_u8()?;

    let c = if a > b { 1 } else { 0 };

    stack.push_operand(c);

    Ok(())
}

fn i32_to_i8(stack: &mut Stack) -> Result {
    let v = stack.pop_operand()?;

    let v = v.to_i32();
    let v: i8 = v.try_into()?;

    stack.push_operand(v);

    Ok(())
}

fn mul_i32(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = a.to_i32();
    let b = b.to_i32();

    let Some(c) = a.checked_mul(b) else {
        return Err(IntegerOverflow.into());
    };

    stack.push_operand(c);

    Ok(())
}

fn mul_u8_wrap(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = a.to_u8()?;
    let b = b.to_u8()?;

    let c = a.wrapping_mul(b);
    stack.push_operand(c);

    Ok(())
}

fn neg_i32(stack: &mut Stack) -> Result {
    let a = stack.pop_operand()?;

    let a = a.to_i32();

    if a == i32::MIN {
        return Err(IntegerOverflow.into());
    }
    let b = -a;

    stack.push_operand(b);

    Ok(())
}

fn not(stack: &mut Stack) -> Result {
    let a = stack.pop_operand()?;

    let b = if a.0 == [0; 4] { 1 } else { 0 };
    stack.push_operand(b);

    Ok(())
}

fn remainder_i32(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = a.to_i32();
    let b = b.to_i32();

    if b == 0 {
        return Err(Effect::DivideByZero);
    }
    let c = a % b;

    stack.push_operand(c);

    Ok(())
}

fn sub_i32(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = a.to_i32();
    let b = b.to_i32();

    let Some(c) = a.checked_sub(b) else {
        return Err(IntegerOverflow.into());
    };

    stack.push_operand(c);

    Ok(())
}

fn sub_u8(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = a.to_u8()?;
    let b = b.to_u8()?;

    let Some(c) = a.checked_sub(b) else {
        return Err(IntegerOverflow.into());
    };

    stack.push_operand(c);

    Ok(())
}

fn sub_u8_wrap(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = a.to_u8()?;
    let b = b.to_u8()?;

    let c = a.wrapping_sub(b);
    stack.push_operand(c);

    Ok(())
}

pub type Result = std::result::Result<(), Effect>;
