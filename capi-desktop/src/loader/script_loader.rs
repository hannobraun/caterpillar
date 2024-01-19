use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use anyhow::Context;
use capi_core::repr::eval::fragments::FragmentId;
use crossbeam_channel::SendError;

use super::{channel::UpdateSender, Update};

pub struct ScriptLoader {
    path: PathBuf,
    parent: Option<FragmentId>,
    sender: UpdateSender,
}

impl ScriptLoader {
    pub fn new(
        path: PathBuf,
        parent: Option<FragmentId>,
        sender: UpdateSender,
    ) -> anyhow::Result<Self> {
        let self_ = Self {
            path,
            parent,
            sender,
        };
        self_.trigger()?;

        Ok(self_)
    }

    pub fn on_error(
        &self,
        err: impl Into<anyhow::Error>,
    ) -> Result<(), SendError<Update>> {
        self.sender.send(Err(err.into()))
    }

    pub fn trigger(&self) -> Result<(), SendError<Update>> {
        let code_or_err = load(&self.path)
            .with_context(|| {
                format!("Loading script `{}`", self.path.display())
            })
            .map(|code| (self.path.clone(), self.parent, code));
        self.sender.send(code_or_err)
    }
}

fn load(path: &Path) -> io::Result<String> {
    let mut code = String::new();
    File::open(path)?.read_to_string(&mut code)?;
    Ok(code)
}
