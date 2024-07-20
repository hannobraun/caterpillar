use std::num::TryFromIntError;

use crate::{
    operands::PopOperandError, stack::PushStackFrameError, Function,
    InstructionAddr, Instructions, Stack,
};

pub fn add(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let Some(c) = a.checked_add(b) else {
        return Err(BuiltinError::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(None)
}

pub fn add_wrap_unsigned(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let c = a.wrapping_add(b);
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

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    if b == 0 {
        return Err(BuiltinError::DivideByZero);
    }
    let Some(c) = a.checked_div(b) else {
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

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let c = if a > b { 1 } else { 0 };

    stack.push_operand(c);

    Ok(None)
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

    Ok(None)
}

pub fn load(stack: &mut Stack) -> Result {
    let address = stack.pop_operand()?;

    let address = i32::from_le_bytes(address.0);
    let address = address.try_into()?;

    Ok(Some(BuiltinEffect::Host(HostEffect::Load { address })))
}

pub fn mul(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let Some(c) = a.checked_mul(b) else {
        return Err(BuiltinError::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(None)
}

pub fn neg(stack: &mut Stack) -> Result {
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);

    if a == i32::MIN {
        return Err(BuiltinError::IntegerOverflow);
    }
    let b = -a;

    stack.push_operand(b);

    Ok(None)
}

pub fn read_input() -> Result {
    Ok(Some(BuiltinEffect::Host(HostEffect::ReadInput)))
}

pub fn read_random() -> Result {
    Ok(Some(BuiltinEffect::Host(HostEffect::ReadRandom)))
}

pub fn remainder(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    if b == 0 {
        return Err(BuiltinError::DivideByZero);
    }
    let c = a % b;

    stack.push_operand(c);

    Ok(None)
}

pub fn set_pixel(stack: &mut Stack) -> Result {
    let a = stack.pop_operand()?;
    let b = stack.pop_operand()?;
    let g = stack.pop_operand()?;
    let r = stack.pop_operand()?;
    let y = stack.pop_operand()?;
    let x = stack.pop_operand()?;

    let x = i32::from_le_bytes(x.0);
    let y = i32::from_le_bytes(y.0);
    let r = i32::from_le_bytes(r.0);
    let g = i32::from_le_bytes(g.0);
    let b = i32::from_le_bytes(b.0);
    let a = i32::from_le_bytes(a.0);

    if x < 0 {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if y < 0 {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if x >= TILES_PER_AXIS_I32 {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if y >= TILES_PER_AXIS_I32 {
        return Err(BuiltinError::OperandOutOfBounds);
    }

    let color_channel_min: i32 = u8::MIN.into();
    let color_channel_max: i32 = u8::MAX.into();

    if r < color_channel_min {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if g < color_channel_min {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if b < color_channel_min {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if a < color_channel_min {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if r > color_channel_max {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if r > color_channel_max {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if r > color_channel_max {
        return Err(BuiltinError::OperandOutOfBounds);
    }
    if r > color_channel_max {
        return Err(BuiltinError::OperandOutOfBounds);
    }

    let [x, y] = [x, y].map(|coord| {
        coord
            .try_into()
            .expect("Just checked that coordinates are within bounds")
    });
    let color = [r, g, b, a].map(|channel| {
        channel
            .try_into()
            .expect("Just checked that color channels are within bounds")
    });

    Ok(Some(BuiltinEffect::Host(HostEffect::SetTile {
        x,
        y,
        color,
    })))
}

pub fn store(stack: &mut Stack) -> Result {
    let address = stack.pop_operand()?;
    let value = stack.pop_operand()?;

    let address = i32::from_le_bytes(address.0);
    let address = address.try_into()?;

    let value = i32::from_le_bytes(value.0);
    let value = value.try_into()?;

    Ok(Some(BuiltinEffect::Host(HostEffect::Store {
        address,
        value,
    })))
}

pub fn sub(stack: &mut Stack) -> Result {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let Some(c) = a.checked_sub(b) else {
        return Err(BuiltinError::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(None)
}

pub fn submit_frame() -> Result {
    Ok(Some(BuiltinEffect::Host(HostEffect::SubmitFrame)))
}

pub type Result = std::result::Result<Option<BuiltinEffect>, BuiltinError>;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum BuiltinEffect {
    Breakpoint,
    Error(BuiltinError),
    Host(HostEffect),
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum HostEffect {
    Load { address: u8 },
    Store { address: u8, value: u8 },

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

    #[error("Integer overflow")]
    IntegerOverflow,

    #[error("Operand is out of bounds")]
    OperandOutOfBounds,

    #[error(transparent)]
    PopOperand(#[from] PopOperandError),

    #[error(transparent)]
    PushStackFrame(#[from] PushStackFrameError),
}

// This conversion is implemented manually, because doing it automatically using
// `thiserror`'s from would add an instance of the error into the type, and it
// doesn't implement `serde::Deserialize`.
impl From<TryFromIntError> for BuiltinError {
    fn from(_: TryFromIntError) -> Self {
        Self::OperandOutOfBounds
    }
}

pub const TILES_PER_AXIS: usize = 32;

// The value is within the bounds of an `i32`. The `as` here should never
// truncate.
const TILES_PER_AXIS_I32: i32 = TILES_PER_AXIS as i32;
