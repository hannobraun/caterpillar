use crossbeam_channel::{SendError, Sender};

pub struct ScriptLoader {
    pub sender: Sender<anyhow::Result<String>>,
}

impl ScriptLoader {
    pub fn new(sender: Sender<anyhow::Result<String>>) -> Self {
        Self { sender }
    }

    pub fn on_error(
        &self,
        err: impl Into<anyhow::Error>,
    ) -> Result<(), SendError<anyhow::Result<String>>> {
        self.sender.send(Err(err.into()))
    }
}
