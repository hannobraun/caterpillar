use std::sync::mpsc;

pub struct EffectsTx {
    pub inner: mpsc::Sender<DisplayEffect>,
}

impl EffectsTx {
    pub fn send(&self, effect: DisplayEffect) {
        // This produces an error, if the other end has hung up, which happens
        // during shutdown. We can safely ignore that.
        let _ = self.inner.send(effect);
    }
}

pub type EffectsRx = mpsc::Receiver<DisplayEffect>;

#[derive(Debug)]
pub enum DisplayEffect {
    SetTile { x: u8, y: u8, value: u8 },
    SubmitTiles { reply: mpsc::Sender<()> },
    ReadInput { reply: mpsc::Sender<i8> },
}
