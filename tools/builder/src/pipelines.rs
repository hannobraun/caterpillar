use std::path::Path;

use crate::{build, serve, watch};

pub async fn runtime() -> anyhow::Result<()> {
    let watcher = watch::Watcher::new(Path::new("capi/runtime"))?;
    let mut updates = build::start(watcher.changes());
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
