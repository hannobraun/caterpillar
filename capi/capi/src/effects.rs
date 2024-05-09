use std::sync::mpsc;

pub struct EffectsTx {
    pub inner: mpsc::Sender<DisplayEffect>,
}

impl EffectsTx {
    pub fn send(&self, effect: DisplayEffect) {
        self.inner.send(effect).unwrap();
    }
}

pub type EffectsRx = mpsc::Receiver<DisplayEffect>;

pub type ResumeTx = mpsc::Sender<()>;
pub type ResumeRx = mpsc::Receiver<()>;

#[derive(Debug)]
pub enum DisplayEffect {
    SetTile { x: u8, y: u8, value: u8 },
    SubmitTiles,
}
