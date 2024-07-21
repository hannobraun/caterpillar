use std::fmt::Debug;

use crate::{builtins, Effect, Stack};

pub trait Host: Clone + Debug + Eq {
    type Effect;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameEngineHost;

impl Host for GameEngineHost {
    type Effect = GameEngineEffect;
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GameEngineEffect {
    Load { address: u8 },
    Store { address: u8, value: u8 },

    SetTile { x: u8, y: u8, color: [u8; 4] },
    SubmitFrame,

    ReadInput,
    ReadRandom,
}

pub fn load(stack: &mut Stack) -> Result {
    let address = stack.pop_operand()?;

    let address = i32::from_le_bytes(address.0);
    let address = address.try_into()?;

    Err(Effect::Host(GameEngineEffect::Load { address }))
}

pub fn read_input() -> Result {
    Err(Effect::Host(GameEngineEffect::ReadInput))
}

pub fn read_random() -> Result {
    Err(Effect::Host(GameEngineEffect::ReadRandom))
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

pub fn store(stack: &mut Stack) -> Result {
    let address = stack.pop_operand()?;
    let value = stack.pop_operand()?;

    let address = i32::from_le_bytes(address.0);
    let address = address.try_into()?;

    let value = i32::from_le_bytes(value.0);
    let value = value.try_into()?;

    Err(Effect::Host(GameEngineEffect::Store { address, value }))
}

pub fn submit_frame() -> Result {
    Err(Effect::Host(GameEngineEffect::SubmitFrame))
}

type Result = builtins::Result<GameEngineHost>;

pub const TILES_PER_AXIS: usize = 32;

// The value is within the bounds of an `i32`. The `as` here should never
// truncate.
const TILES_PER_AXIS_I32: i32 = TILES_PER_AXIS as i32;
