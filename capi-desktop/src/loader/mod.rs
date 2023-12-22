mod load;
mod watch;

pub use self::{load::load, watch::watch};

use std::path::Path;

pub struct Loader;

impl Loader {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<String> {
        self::load::load(path)
    }
}
