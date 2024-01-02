use std::path::PathBuf;

use crossbeam_channel::{Receiver, SendError, Sender};

use super::load;

pub struct ScriptLoader {
    path: PathBuf,
    sender: Sender<anyhow::Result<String>>,
}

impl ScriptLoader {
    pub fn new(
        path: PathBuf,
    ) -> anyhow::Result<(Self, Receiver<anyhow::Result<String>>)> {
        // The channel buffer is size 1, so the initial call to `trigger` within
        // this constructor is guaranteed not to block. Subsequent calls may
        // block, as per the documentation of that method.
        let (sender, receiver) = crossbeam_channel::bounded(1);

        let self_ = Self { path, sender };
        self_.trigger()?;

        Ok((self_, receiver))
    }

    pub fn on_error(
        &self,
        err: impl Into<anyhow::Error>,
    ) -> Result<(), SendError<anyhow::Result<String>>> {
        self.sender.send(Err(err.into()))
    }

    /// Trigger a code update
    ///
    /// This method may block indefinitely while waiting for the code update to
    /// be processed!
    pub fn trigger(&self) -> Result<(), SendError<anyhow::Result<String>>> {
        let code_or_err = load(&self.path);
        self.sender.send(code_or_err)
    }
}
