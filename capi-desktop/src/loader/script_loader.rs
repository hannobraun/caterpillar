use std::path::PathBuf;

use crossbeam_channel::{SendError, Sender};

use super::load;

pub struct ScriptLoader {
    path: PathBuf,
    sender: Sender<anyhow::Result<String>>,
}

impl ScriptLoader {
    pub fn new(path: PathBuf, sender: Sender<anyhow::Result<String>>) -> Self {
        Self { path, sender }
    }

    pub fn on_error(
        &self,
        err: impl Into<anyhow::Error>,
    ) -> Result<(), SendError<anyhow::Result<String>>> {
        self.sender.send(Err(err.into()))
    }

    pub fn on_change(&self) -> Result<(), SendError<anyhow::Result<String>>> {
        let code_or_err = load(&self.path);
        self.sender.send(code_or_err)
    }
}
