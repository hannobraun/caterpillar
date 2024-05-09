use std::sync::mpsc;

pub type EffectsTx = mpsc::Sender<DisplayEffect>;
pub type EffectsRx = mpsc::Receiver<DisplayEffect>;

pub type ResumeTx = mpsc::Sender<()>;
pub type ResumeRx = mpsc::Receiver<()>;

#[derive(Debug)]
pub enum DisplayEffect {
    SetTile { x: u8, y: u8, value: u8 },
    SubmitTiles,
}
