use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use anyhow::Context;
use crossbeam_channel::SendError;

use super::{Update, UpdateSender};

pub struct ScriptLoader {
    path: PathBuf,
    sender: UpdateSender,
}

impl ScriptLoader {
    pub fn new(path: PathBuf, sender: UpdateSender) -> anyhow::Result<Self> {
        let self_ = Self { path, sender };
        self_.trigger()?;

        Ok(self_)
    }

    pub fn on_error(
        &self,
        err: impl Into<anyhow::Error>,
    ) -> Result<(), SendError<Update>> {
        self.sender.send(Err(err.into()))
    }

    /// Trigger a code update
    ///
    /// This method may block indefinitely while waiting for the code update to
    /// be processed!
    pub fn trigger(&self) -> Result<(), SendError<Update>> {
        let code_or_err = load(&self.path).with_context(|| {
            format!("Loading script `{}`", self.path.display())
        });
        self.sender.send(code_or_err)
    }
}

fn load(path: &Path) -> io::Result<String> {
    let mut code = String::new();
    File::open(path)?.read_to_string(&mut code)?;
    Ok(code)
}
