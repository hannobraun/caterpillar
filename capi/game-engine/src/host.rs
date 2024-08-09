use capi_process::Host;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GameEngineHost;

impl Host for GameEngineHost {
    type Effect = GameEngineEffect;

    fn function(name: &str) -> Option<Self::Effect> {
        match name {
            "load" => Some(GameEngineEffect::Load),
            "read_input" => Some(GameEngineEffect::ReadInput),
            "read_random" => Some(GameEngineEffect::ReadRandom),
            "set_pixel" => Some(GameEngineEffect::SetPixel),
            "store" => Some(GameEngineEffect::Store),
            "submit_frame" => Some(GameEngineEffect::SubmitFrame),
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
