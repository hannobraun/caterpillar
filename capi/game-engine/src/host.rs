use capi_process::{Effect, Host, HostFunction, Stack};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GameEngineHost;

impl Host for GameEngineHost {
    type Effect = GameEngineEffect;

    fn function(name: &str) -> Option<HostFunction<Self::Effect>> {
        match name {
            "load" => Some(load),
            "read_input" => Some(read_input),
            "read_random" => Some(read_random),
            "set_pixel" => Some(set_pixel),
            "store" => Some(store),
            "submit_frame" => Some(submit_frame),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GameEngineEffect {
    Load,
    Store { address: u8, value: u8 },

    ReadInput,
    ReadRandom,

    SetPixel,
    SubmitFrame,
}

pub fn load(_: &mut Stack) -> GameEngineResult {
    Err(Effect::Host(GameEngineEffect::Load))
}

pub fn read_input(_: &mut Stack) -> GameEngineResult {
    Err(Effect::Host(GameEngineEffect::ReadInput))
}

pub fn read_random(_: &mut Stack) -> GameEngineResult {
    Err(Effect::Host(GameEngineEffect::ReadRandom))
}

pub fn set_pixel(_: &mut Stack) -> GameEngineResult {
    Err(Effect::Host(GameEngineEffect::SetPixel))
}

pub fn store(stack: &mut Stack) -> GameEngineResult {
    let address = stack.pop_operand()?;
    let value = stack.pop_operand()?;

    let address = i32::from_le_bytes(address.0);
    let address = address.try_into()?;

    let value = i32::from_le_bytes(value.0);
    let value = value.try_into()?;

    Err(Effect::Host(GameEngineEffect::Store { address, value }))
}

pub fn submit_frame(_: &mut Stack) -> GameEngineResult {
    Err(Effect::Host(GameEngineEffect::SubmitFrame))
}

type GameEngineResult = Result<(), Effect<GameEngineEffect>>;

pub const TILES_PER_AXIS: usize = 32;

// The value is within the bounds of an `i32`. The `as` here should never
// truncate.
pub const TILES_PER_AXIS_I32: i32 = TILES_PER_AXIS as i32;
