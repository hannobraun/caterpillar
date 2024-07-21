use std::fmt::Debug;

pub trait Host: Clone + Debug + Eq {
    type Effect;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameEngineHost;

impl Host for GameEngineHost {
    type Effect = HostEffect;
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
