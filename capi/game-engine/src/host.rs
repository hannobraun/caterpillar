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
    Store,

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

pub fn store(_: &mut Stack) -> GameEngineResult {
    Err(Effect::Host(GameEngineEffect::Store))
}

pub fn submit_frame(_: &mut Stack) -> GameEngineResult {
    Err(Effect::Host(GameEngineEffect::SubmitFrame))
}

type GameEngineResult = Result<(), Effect<GameEngineEffect>>;

pub const TILES_PER_AXIS: u8 = 32;
