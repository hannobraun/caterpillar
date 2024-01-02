use std::path::Path;

use crossbeam_channel::{SendError, Sender};

use super::load;

pub struct ScriptLoader {
    sender: Sender<anyhow::Result<String>>,
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

    pub fn on_change(
        &self,
        path: &Path,
    ) -> Result<(), SendError<anyhow::Result<String>>> {
        let code_or_err = load(path);
        self.sender.send(code_or_err)
    }
}
