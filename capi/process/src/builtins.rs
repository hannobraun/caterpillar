use crate::{
    host::GameEngineHost, Effect, Function, GameEngineEffect, InstructionAddr,
    Instructions, Stack,
};

pub fn add(stack: &mut Stack) -> Result<GameEngineHost> {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let Some(c) = a.checked_add(b) else {
        return Err(Effect::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(())
}

pub fn add_wrap_unsigned(stack: &mut Stack) -> Result<GameEngineHost> {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let c = a.wrapping_add(b);
    let c = if c >= 0 { c } else { c - i32::MIN };

    stack.push_operand(c);

    Ok(())
}

pub fn brk() -> Result<GameEngineHost> {
    Err(Effect::Breakpoint)
}

pub fn copy(stack: &mut Stack) -> Result<GameEngineHost> {
    let a = stack.pop_operand()?;

    stack.push_operand(a);
    stack.push_operand(a);

    Ok(())
}

pub fn div(stack: &mut Stack) -> Result<GameEngineHost> {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    if b == 0 {
        return Err(Effect::DivideByZero);
    }
    let Some(c) = a.checked_div(b) else {
        // Can't be divide by zero. Already handled that.
        return Err(Effect::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(())
}

pub fn drop(stack: &mut Stack) -> Result<GameEngineHost> {
    stack.pop_operand()?;
    Ok(())
}

pub fn eq(stack: &mut Stack) -> Result<GameEngineHost> {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let c = if a.0 == b.0 { 1 } else { 0 };

    stack.push_operand(c);

    Ok(())
}

pub fn greater(stack: &mut Stack) -> Result<GameEngineHost> {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let c = if a > b { 1 } else { 0 };

    stack.push_operand(c);

    Ok(())
}

pub fn if_(
    stack: &mut Stack,
    instructions: &Instructions,
) -> Result<GameEngineHost> {
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

pub fn load(stack: &mut Stack) -> Result<GameEngineHost> {
    let address = stack.pop_operand()?;

    let address = i32::from_le_bytes(address.0);
    let address = address.try_into()?;

    Err(Effect::Host(GameEngineEffect::Load { address }))
}

pub fn mul(stack: &mut Stack) -> Result<GameEngineHost> {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let Some(c) = a.checked_mul(b) else {
        return Err(Effect::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(())
}

pub fn neg(stack: &mut Stack) -> Result<GameEngineHost> {
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);

    if a == i32::MIN {
        return Err(Effect::IntegerOverflow);
    }
    let b = -a;

    stack.push_operand(b);

    Ok(())
}

pub fn read_input() -> Result<GameEngineHost> {
    Err(Effect::Host(GameEngineEffect::ReadInput))
}

pub fn read_random() -> Result<GameEngineHost> {
    Err(Effect::Host(GameEngineEffect::ReadRandom))
}

pub fn remainder(stack: &mut Stack) -> Result<GameEngineHost> {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    if b == 0 {
        return Err(Effect::DivideByZero);
    }
    let c = a % b;

    stack.push_operand(c);

    Ok(())
}

pub fn set_pixel(stack: &mut Stack) -> Result<GameEngineHost> {
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
        return Err(Effect::OperandOutOfBounds);
    }
    if y < 0 {
        return Err(Effect::OperandOutOfBounds);
    }
    if x >= TILES_PER_AXIS_I32 {
        return Err(Effect::OperandOutOfBounds);
    }
    if y >= TILES_PER_AXIS_I32 {
        return Err(Effect::OperandOutOfBounds);
    }

    let color_channel_min: i32 = u8::MIN.into();
    let color_channel_max: i32 = u8::MAX.into();

    if r < color_channel_min {
        return Err(Effect::OperandOutOfBounds);
    }
    if g < color_channel_min {
        return Err(Effect::OperandOutOfBounds);
    }
    if b < color_channel_min {
        return Err(Effect::OperandOutOfBounds);
    }
    if a < color_channel_min {
        return Err(Effect::OperandOutOfBounds);
    }
    if r > color_channel_max {
        return Err(Effect::OperandOutOfBounds);
    }
    if r > color_channel_max {
        return Err(Effect::OperandOutOfBounds);
    }
    if r > color_channel_max {
        return Err(Effect::OperandOutOfBounds);
    }
    if r > color_channel_max {
        return Err(Effect::OperandOutOfBounds);
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

    Err(Effect::Host(GameEngineEffect::SetTile { x, y, color }))
}

pub fn store(stack: &mut Stack) -> Result<GameEngineHost> {
    let address = stack.pop_operand()?;
    let value = stack.pop_operand()?;

    let address = i32::from_le_bytes(address.0);
    let address = address.try_into()?;

    let value = i32::from_le_bytes(value.0);
    let value = value.try_into()?;

    Err(Effect::Host(GameEngineEffect::Store { address, value }))
}

pub fn sub(stack: &mut Stack) -> Result<GameEngineHost> {
    let b = stack.pop_operand()?;
    let a = stack.pop_operand()?;

    let a = i32::from_le_bytes(a.0);
    let b = i32::from_le_bytes(b.0);

    let Some(c) = a.checked_sub(b) else {
        return Err(Effect::IntegerOverflow);
    };

    stack.push_operand(c);

    Ok(())
}

pub fn submit_frame() -> Result<GameEngineHost> {
    Err(Effect::Host(GameEngineEffect::SubmitFrame))
}

pub type Result<H> = std::result::Result<(), Effect<H>>;

pub const TILES_PER_AXIS: usize = 32;

// The value is within the bounds of an `i32`. The `as` here should never
// truncate.
const TILES_PER_AXIS_I32: i32 = TILES_PER_AXIS as i32;
