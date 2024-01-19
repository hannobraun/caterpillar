use capi_core::repr::eval::fragments::FragmentId;
use crossbeam_channel::{Receiver, Sender};

pub type Update = anyhow::Result<(Option<FragmentId>, String)>;
pub type UpdateSender = Sender<Update>;
pub type UpdateReceiver = Receiver<Update>;
