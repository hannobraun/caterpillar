use std::num::TryFromIntError;

use super::{Stack, StackUnderflow, Value};

pub fn add(stack: &mut Stack) -> Result {
    let b = stack.operands_mut().pop()?;
    let a = stack.operands_mut().pop()?;

    let Some(c) = a.0.checked_add(b.0) else {
        return Err(BuiltinError::IntegerOverflow);
    };

    stack.operands_mut().push(c);

    Ok(None)
}

pub fn add_wrap_unsigned(stack: &mut Stack) -> Result {
    let b = stack.operands_mut().pop()?;
    let a = stack.operands_mut().pop()?;

    let c = a.0.wrapping_add(b.0);
    let c = if c >= 0 { c } else { c - i8::MIN };

    stack.operands_mut().push(c);

    Ok(None)
}

pub fn brk() -> Result {
    Ok(Some(BuiltinEffect::Breakpoint))
}

pub fn copy(stack: &mut Stack) -> Result {
    let a = stack.operands_mut().pop()?;

    stack.operands_mut().push(a);
    stack.operands_mut().push(a);

    Ok(None)
}

pub fn div(stack: &mut Stack) -> Result {
    let b = stack.operands_mut().pop()?;
    let a = stack.operands_mut().pop()?;

    if b.0 == 0 {
        return Err(BuiltinError::DivideByZero);
    }
    let Some(c) = a.0.checked_div(b.0) else {
        // Can't be divide by zero. Already handled that.
        return Err(BuiltinError::IntegerOverflow);
    };

    stack.operands_mut().push(c);

    Ok(None)
}

pub fn drop(stack: &mut Stack) -> Result {
    stack.operands_mut().pop()?;
    Ok(None)
}

pub fn eq(stack: &mut Stack) -> Result {
    let b = stack.operands_mut().pop()?;
    let a = stack.operands_mut().pop()?;

    let c = if a.0 == b.0 { 1 } else { 0 };

    stack.operands_mut().push(c);

    Ok(None)
}

pub fn greater(stack: &mut Stack) -> Result {
    let b = stack.operands_mut().pop()?;
    let a = stack.operands_mut().pop()?;

    let c = if a.0 > b.0 { 1 } else { 0 };

    stack.operands_mut().push(c);

    Ok(None)
}

pub fn load(stack: &mut Stack) -> Result {
    let address = stack.operands_mut().pop()?;

    let address = address.0.try_into()?;

    Ok(Some(BuiltinEffect::Load { address }))
}

pub fn mul(stack: &mut Stack) -> Result {
    let b = stack.operands_mut().pop()?;
    let a = stack.operands_mut().pop()?;

    let Some(c) = a.0.checked_mul(b.0) else {
        return Err(BuiltinError::IntegerOverflow);
    };

    stack.operands_mut().push(c);

    Ok(None)
}

pub fn neg(stack: &mut Stack) -> Result {
    let a = stack.operands_mut().pop()?;

    if a.0 == i8::MIN {
        return Err(BuiltinError::IntegerOverflow);
    }
    let b = -a.0;

    stack.operands_mut().push(b);

    Ok(None)
}

pub fn read_input() -> Result {
    Ok(Some(BuiltinEffect::ReadInput))
}

pub fn read_random() -> Result {
    Ok(Some(BuiltinEffect::ReadRandom))
}

pub fn remainder(stack: &mut Stack) -> Result {
    let b = stack.operands_mut().pop()?;
    let a = stack.operands_mut().pop()?;

    if b.0 == 0 {
        return Err(BuiltinError::DivideByZero);
    }
    let c = a.0 % b.0;

    stack.operands_mut().push(c);

    Ok(None)
}

pub fn store(stack: &mut Stack) -> Result {
    let address = stack.operands_mut().pop()?;
    let value = stack.operands_mut().pop()?;

    let address = address.0.try_into()?;

    Ok(Some(BuiltinEffect::Store { address, value }))
}

pub fn sub(stack: &mut Stack) -> Result {
    let b = stack.operands_mut().pop()?;
    let a = stack.operands_mut().pop()?;

    let Some(c) = a.0.checked_sub(b.0) else {
        return Err(BuiltinError::IntegerOverflow);
    };

    stack.operands_mut().push(c);

    Ok(None)
}

pub fn submit_frame() -> Result {
    Ok(Some(BuiltinEffect::SubmitFrame))
}

pub fn write_tile(stack: &mut Stack) -> Result {
    let value = stack.operands_mut().pop()?;
    let y = stack.operands_mut().pop()?;
    let x = stack.operands_mut().pop()?;

    let x = x.0.try_into()?;
    let y = y.0.try_into()?;
    let value = value.0.try_into()?;

    Ok(Some(BuiltinEffect::SetTile { x, y, value }))
}

pub type Result = std::result::Result<Option<BuiltinEffect>, BuiltinError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BuiltinEffect {
    Breakpoint,
    Error(BuiltinError),

    Load { address: u8 },
    Store { address: u8, value: Value },

    SetTile { x: u8, y: u8, value: u8 },
    SubmitFrame,

    ReadInput,
    ReadRandom,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum BuiltinError {
    #[error("Divide by zero")]
    DivideByZero,

    #[error("Expected positive value")]
    ExpectedPositiveValue,

    #[error("Integer overflow")]
    IntegerOverflow,

    #[error(transparent)]
    StackUnderflow(#[from] StackUnderflow),
}

// This conversion is implemented manually, because doing it automatically using
// `thiserror`'s from would add an instance of the error into the type, and it
// doesn't implement `serde::Deserialize`.
impl From<TryFromIntError> for BuiltinError {
    fn from(_: TryFromIntError) -> Self {
        Self::ExpectedPositiveValue
    }
}
