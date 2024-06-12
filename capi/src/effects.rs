use tokio::sync::mpsc;

pub type EffectsRx = mpsc::UnboundedReceiver<DisplayEffect>;

#[derive(Debug)]
pub enum DisplayEffect {}
