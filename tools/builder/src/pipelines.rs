use std::path::PathBuf;

use crate::{build, serve, watch};

pub async fn pipeline() -> anyhow::Result<()> {
    let crates_dir = PathBuf::from("capi").canonicalize()?;

    let watcher = watch::Watcher::new(crates_dir)?;
    let mut updates = build::start(watcher.changes);
    let address = serve::start(updates.clone()).await?;

    while let Ok(()) = updates.changed().await {
        println!();
        println!("Caterpillar is ready:");
        println!();
        println!("\t🚀 http://{address}/");
        println!();
    }

    Ok(())
}
