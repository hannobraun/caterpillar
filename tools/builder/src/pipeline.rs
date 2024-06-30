use std::path::PathBuf;

use capi_watch::Watcher;

use crate::{build, serve};

pub async fn pipeline() -> anyhow::Result<()> {
    let crates_dir = PathBuf::from("capi").canonicalize()?;

    let watcher = Watcher::new(crates_dir)?;
    let updates = build::start(watcher.changes);
    serve::start(updates).await?;

    Ok(())
}
