use std::path::Path;

use crate::{build, serve, watch};

pub async fn runtime() -> anyhow::Result<()> {
    let runtime_path = Path::new("capi");

    let watcher = watch::Watcher::new(runtime_path)?;
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
