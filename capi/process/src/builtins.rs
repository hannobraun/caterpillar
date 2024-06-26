use std::num::TryFromIntError;

use crate::{operands::PopOperandError, Stack, Value};

pub fn add(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let Some(c) = a.0.checked_add(b.0) else {
        return Err(BuiltinError::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(None)
}

pub fn add_wrap_unsigned(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let c = a.0.wrapping_add(b.0);
    let c = if c >= 0 { c } else { c - i32::MIN };

    stack.push_operand(c);

    Ok(None)
}

pub fn brk() -> Result {
    Ok(Some(BuiltinEffect::Breakpoint))
}

pub fn copy(stack: &mut Stack) -> Result {
    let a = stack.pop_operand()?;

    stack.push_operand(a);
    stack.push_operand(a);

    Ok(None)
}

pub fn div(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    if b.0 == 0 {
        return Err(BuiltinError::DivideByZero);
    }
    let Some(c) = a.0.checked_div(b.0) else {
        // Can't be divide by zero. Already handled that.
        return Err(BuiltinError::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(None)
}

pub fn drop(stack: &mut Stack) -> Result {
    stack.pop_operand()?;
    Ok(None)
}

pub fn eq(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let c = if a.0 == b.0 { 1 } else { 0 };

    stack.push_operand(c);

    Ok(None)
}

pub fn greater(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let c = if a.0 > b.0 { 1 } else { 0 };

    stack.push_operand(c);

    Ok(None)
}

pub fn load(stack: &mut Stack) -> Result {
    let address = stack.pop_operand()?;

    let address = address.0.try_into()?;

    Ok(Some(BuiltinEffect::Load { address }))
}

pub fn mul(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let Some(c) = a.0.checked_mul(b.0) else {
        return Err(BuiltinError::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(None)
}

pub fn neg(stack: &mut Stack) -> Result {
    let a = stack.pop_operand()?;

    if a.0 == i32::MIN {
        return Err(BuiltinError::IntegerOverflow);
    }
    let b = -a.0;

    stack.push_operand(b);

    Ok(None)
}

pub fn read_input() -> Result {
    Ok(Some(BuiltinEffect::ReadInput))
}

pub fn read_random() -> Result {
    Ok(Some(BuiltinEffect::ReadRandom))
}

pub fn remainder(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    if b.0 == 0 {
        return Err(BuiltinError::DivideByZero);
    }
    let c = a.0 % b.0;

    stack.push_operand(c);

    Ok(None)
}

pub fn store(stack: &mut Stack) -> Result {
    let address = stack.pop_operand()?;
    let value = stack.pop_operand()?;

    let address = address.0.try_into()?;

    Ok(Some(BuiltinEffect::Store { address, value }))
}

pub fn sub(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let Some(c) = a.0.checked_sub(b.0) else {
        return Err(BuiltinError::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(None)
}

pub fn submit_frame() -> Result {
    Ok(Some(BuiltinEffect::SubmitFrame))
}

pub fn write_tile(stack: &mut Stack) -> Result {
    let value = stack.pop_operand()?;
    let y = stack.pop_operand()?;
    let x = stack.pop_operand()?;

    if x.0 < 0 {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if y.0 < 0 {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if x.0 >= TILES_PER_AXIS_I32 {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if y.0 >= TILES_PER_AXIS_I32 {
        return Err(BuiltinError::OperandOutOfBounds);
    }

    let [x, y] = [x, y].map(|coord| {
        coord
            .0
            .try_into()
            .expect("Just checked that coordinates are within bounds")
    });

    let color = if value.0 == 0 {
        [0, 0, 0, 255]
    } else {
        [255, 255, 255, 255]
    };

    Ok(Some(BuiltinEffect::SetTile { x, y, color }))
}

pub type Result = std::result::Result<Option<BuiltinEffect>, BuiltinError>;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum BuiltinEffect {
    Breakpoint,
    Error(BuiltinError),

    Load { address: u8 },
    Store { address: u8, value: Value },

    SetTile { x: u8, y: u8, color: [u8; 4] },
    SubmitFrame,

    ReadInput,
    ReadRandom,
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum BuiltinError {
    #[error("Divide by zero")]
    DivideByZero,

    #[error("Expected positive value")]
    ExpectedPositiveValue,

    #[error("Integer overflow")]
    IntegerOverflow,

    #[error("Operand is out of bounds")]
    OperandOutOfBounds,

    #[error(transparent)]
    PopOperand(#[from] PopOperandError),
}

// This conversion is implemented manually, because doing it automatically using
// `thiserror`'s from would add an instance of the error into the type, and it
// doesn't implement `serde::Deserialize`.
impl From<TryFromIntError> for BuiltinError {
    fn from(_: TryFromIntError) -> Self {
        Self::ExpectedPositiveValue
    }
}

pub const TILES_PER_AXIS: usize = 32;

// The value is within the bounds of an `i32`. The `as` here should never
// truncate.
const TILES_PER_AXIS_I32: i32 = TILES_PER_AXIS as i32;
