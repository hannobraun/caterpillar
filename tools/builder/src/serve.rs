use tokio::process::{Child, Command};

use crate::build::UpdatesRx;

pub async fn start(mut updates: UpdatesRx) -> anyhow::Result<()> {
    let address = "localhost:34480";

    let mut current_server: Option<Child> = None;

    updates.mark_unchanged(); // make sure we enter the loop body immediately
    while let Ok(()) = updates.changed().await {
        let Some(serve_dir) = &*updates.borrow() else {
            continue;
        };

        if let Some(mut server) = current_server.take() {
            server.kill().await?;
        }

        let new_server = Command::new("cargo")
            .arg("run")
            .args(["--package", "capi-server"])
            .arg("--")
            .args(["--address", address])
            .args(["--serve-dir", &serve_dir.display().to_string()])
            .spawn()?;

        current_server = Some(new_server);

        println!();
        println!("✅ Build is ready:");
        println!();
        println!("\t🚀 http://{address}/");
        println!();
        println!("------------------------------------------------");
        println!();
    }

    Ok(())
}
