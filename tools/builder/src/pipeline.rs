use std::path::PathBuf;

use crate::{build, serve, watch};

pub async fn pipeline() -> anyhow::Result<()> {
    let crates_dir = PathBuf::from("capi").canonicalize()?;

    let watcher = watch::Watcher::new(crates_dir)?;
    let updates = build::start(watcher.changes);
    serve::start(updates.clone()).await?;

    Ok(())
}
