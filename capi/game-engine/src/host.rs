use capi_compiler::host::Host;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GameEngineHost;

impl Host for GameEngineHost {
    fn function_name_to_effect_number(name: &str) -> Option<u8> {
        let effect = match name {
            "load" => GameEngineEffect::Load,
            "read_input" => GameEngineEffect::ReadInput,
            "read_random" => GameEngineEffect::ReadRandom,
            "set_pixel" => GameEngineEffect::SetPixel,
            "store" => GameEngineEffect::Store,
            "submit_frame" => GameEngineEffect::SubmitFrame,

            _ => {
                return None;
            }
        };

        Some(effect.into())
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
    serde::Deserialize,
    serde::Serialize,
)]
#[repr(u8)]
pub enum GameEngineEffect {
    Load,
    Store,

    ReadInput,
    ReadRandom,

    SetPixel,
    SubmitFrame,
}
